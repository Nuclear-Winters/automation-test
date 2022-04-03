#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use cocoon::{Cocoon, Creation};
use ssh2::{Channel, Session};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread::spawn;
use std::time::Duration;
use tauri::Window;

use crate::config_data::{Config, Server, ServerName};

mod config_data;
mod config_update;
mod return_error_macro;

//Config location.
const CONFIG_LOCATION: &str = "./config.crypt";

pub struct State {
    pub config: RwLock<Option<Config>>, //The config is in a lock so an infinite amount of threads can read it, but only one can write
    pub completed_tasks: RwLock<HashMap<ServerName,Vec<String>>>, //Holds a list of server names that have ran their command and returned an output
}
#[tauri::command]
fn get_task(server_name: String, state: tauri::State<Arc<State>>) -> Result<String, String>{
    let mut task_lock = state.completed_tasks.write().unwrap();
    return if task_lock.contains_key(&server_name) {
        let task = task_lock.remove(&server_name).unwrap();
        let mut data = String::new();
        for result in task {
            data += &format!("{} \n", result)
        }
        Ok(data)
    } else {
        Ok("".to_string())
    }
}

#[tauri::command]
fn add_group(name: String, state: tauri::State<Arc<State>>) -> Result<(), String> {
    let mut config_lock = return_error!(state.config.write());

    if let Some(config) = &mut *config_lock {
        config.groups.insert(name, vec![]);
    }
    Ok(())
}

#[tauri::command]
fn server_execute_command(
    command: String,
    pre_cmd: Option<String>,
    server: String,
    state: tauri::State<Arc<State>>,
) -> Result<(), String> {
    let config_lock = return_error!(state.config.read());
    let config = match &*config_lock {
        None => { return Err("No Config loaded".to_string()) }
        Some(val) => {val}
    };

    let server_data = match config.servers.get(&server) {
        None => { return Err("Server does not exist".to_string()) }
        Some(data) => {data.clone()}
    };

    //If the preset command exist we take whatever command it has and load it into the ssh function
    if let Some(command) = pre_cmd {
        if let Some(command_to_use) = config.pre_commands.get(&command) {
            execute_ssh_command(state.inner().clone(),command_to_use.to_string(),server_data, server);
            return Ok(())
        }
        return Err("Preset command does not exist".to_string())
    }
    else {
        execute_ssh_command(state.inner().clone(),command,server_data, server);
    }

    Ok(())
}

/// A function that passes a new thread all the info it needs to execute the ssh command
/// the result will be passed to the completed task list and the thread will end
pub fn execute_ssh_command(
    state: Arc<State>,
    command: String,
    server_data: Server,
    server_name: String,
) {
    let _ = spawn(move || {
        //This block connects to the server and tries to initialize SSH
        let tcp: TcpStream = return_task_error!(TcpStream::connect(server_data.ip), state, server_name);
        let mut sess: Session = return_task_error!(Session::new(), state, server_name);
        sess.set_tcp_stream(tcp);
        return_task_error!(sess.handshake(), state, server_name);

        //This one function block authenticates with the server its connecting to using the data provided
        return_task_error!(
            sess.userauth_pubkey_memory(
                &server_data.user,
                server_data.public.as_deref(),
                &server_data.private,
                server_data.passphrase.as_deref()
            ),
            state,
            server_name
        );

        //This block executes the command and waits for an output from the remote host
        let mut com_channel: Channel = return_task_error!(sess.channel_session(), state, server_name);
        return_task_error!(com_channel.exec(&command), state, server_name);
        let mut output = String::new();
        return_task_error!(com_channel.read_to_string(&mut output), state, server_name);
        com_channel.wait_close();

        let mut task_lock = state.completed_tasks.write().unwrap();

        if task_lock.contains_key(&server_name) {
            task_lock.get_mut(&server_name).unwrap().push(output);
        }
        else {
            task_lock.insert(server_name,vec![output]);
        }
    });
}

///loads a new server into the config
#[tauri::command]
fn add_server(
    name: String,
    ip: SocketAddr,
    private: String,
    public: Option<String>,
    user: String,
    pass: Option<String>,
    state: tauri::State<Arc<State>>,
) -> Result<(), String> {
    let mut config_lock = return_error!(state.config.write());
    //Create server struct from the data
    let server = Server::new(ip, private, public, user, pass);

    //Access servers hashmap and insert the new server
    if let Some(config) = &mut *config_lock {
        config.servers.insert(name, server);
    }

    Ok(())
}

///For returning users, loads the config from disk with the password
#[tauri::command]
fn load_config(pass: String, state: tauri::State<Arc<State>>,window: Window) -> Result<(), String> {
    let cocoon = Cocoon::new(pass.as_bytes());

    //Read raw config data
    let mut file: File = return_error!(File::open(CONFIG_LOCATION));
    let mut data: Vec<u8> = vec![];
    return_error!(file.read_to_end(&mut data));

    //decrypt data and deserialize into a config structure
    let decrypted_data: Vec<u8> = match cocoon.unwrap(&data) {
        Ok(val) => val,
        Err(_) => return Err("unable to decrypt data. Wrong Password?".to_string()),
    };
    let config: Config = return_error!(bincode::deserialize(&decrypted_data));

    //Get write access to the state and add the config
    let mut conf_lock = return_error!(state.config.write());
    *conf_lock = Some(config);
    config_update::config_update_thread(state.inner().clone(), pass);
    window.emit("change_comp","HOMEPG");
    Ok(())
}

///For intial setup, stores the new config on disk and then throws it to the state
#[tauri::command]
fn create_config(pass: String, state: tauri::State<Arc<State>>,window: Window) -> Result<(), String> {
    let cocoon = Cocoon::new(pass.as_bytes()); //Encrypted object  to store your config on disk
    let conf: Config = Config::default();

    //Turn the config into bytes and encrypt it
    let serialized_data: Vec<u8> = return_error!(bincode::serialize(&conf));
    let encrypted_data: Vec<u8> = match cocoon.wrap(&serialized_data) {
        Ok(val) => val,
        Err(_) => return Err("unable to encrypt data".to_string()),
    };

    //Create a new file and write the encrypted data to it
    let mut file: File = return_error!(File::create(CONFIG_LOCATION));
    return_error!(file.write_all(&encrypted_data));
    return_error!(file.flush());

    let mut config_lock = return_error!(state.config.write());
    *config_lock = Some(conf);
    config_update::config_update_thread(state.inner().clone(), pass);
    window.emit("change_comp","HOMEPG");
    Ok(())
}

#[tauri::command]
fn config_exists() -> bool{
    std::path::Path::new(CONFIG_LOCATION).exists()
}

fn main() {
    //The state is held in the main method and its reference is allowed to be held by any thread
    //This is great for multi-threaded programs that require access to state
    let state_prime = Arc::new(State {
        config: RwLock::default(),
        completed_tasks: Default::default(),
    });

    let state_tauri = state_prime;
    tauri::Builder::default()
        .manage(state_tauri)
        .invoke_handler(tauri::generate_handler![
            create_config,
            load_config,
            add_server,
            add_group,
            server_execute_command,
            config_exists,
            get_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
