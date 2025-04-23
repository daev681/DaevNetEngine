use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: u16,
    pub buffer_size: usize,
}

impl Config {
    pub fn load_config() -> Config {
        Config {
            server_ip: "127.0.0.1".to_string(),
            server_port: 8080,
            buffer_size: 1024,
        }
    }
}