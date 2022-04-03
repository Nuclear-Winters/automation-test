use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;

//Simply here for neatness
type CommandName = String;
type GroupName = String;
pub type ServerName = String;

///A Config struct that has all the groups and servers
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub servers: HashMap<ServerName, Server>,
    pub groups: HashMap<GroupName, Vec<ServerName>>,
    pub pre_commands: HashMap<CommandName, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            servers: Default::default(),
            groups: HashMap::from([("All".to_string(), vec![])]),
            pre_commands: Default::default(),
        }
    }
}

///A server, pretty simple.
#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub user: String,
    pub ip: SocketAddr,
    pub private: String,
    pub public: Option<String>,
    pub passphrase: Option<String>,
}

impl Server {
    pub fn new(
        ip: SocketAddr,
        private: String,
        public: Option<String>,
        user: String,
        passphrase: Option<String>,
    ) -> Self {
        Self {
            user,
            ip,
            private,
            public,
            passphrase,
        }
    }
}
