use crate::{State, CONFIG_LOCATION};
use cocoon::Cocoon;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

/// This function creates a new thread that will periodically (every 5 seconds)
/// convert the current config into bytes, encrypt it, and save to a file
/// basically autosave
pub fn config_update_thread(state: Arc<State>, pass: String) {
    spawn(move || {
        let encrypt = Cocoon::new(pass.as_bytes()); //This has to be made inside the thread due to thread safety checks (its a rust thing)
        let state = state; //This moves the state arc into the thread
        loop {
            std::thread::sleep(Duration::from_secs(5));

            if let Ok(conf) = state.config.read() {
                if let Some(config) = &*conf {
                    let serialized_data: Vec<u8> = match bincode::serialize(&config) {
                        Ok(val) => val,
                        Err(_) => {
                            continue;
                        }
                    };
                    let encrypted_data: Vec<u8> = match encrypt.wrap(&serialized_data) {
                        Ok(val) => val,
                        Err(_) => {
                            continue;
                        }
                    };

                    //Create a new file and write the encrypted data to it
                    let mut file: File = match File::create(CONFIG_LOCATION) {
                        Ok(var) => var,
                        Err(_) => {
                            continue;
                        }
                    };
                    let _ = file.write_all(&encrypted_data);
                    let _ = file.flush(); //im using _ variable to make clippy (the rust linter) shut up
                }
            }
        }
    });
}
