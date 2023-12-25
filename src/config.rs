use serde::Deserialize;
use serde_yaml::{from_str, Value};
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Networking {
    pub online_mode: bool,
    pub keep_alive: bool,
    pub latency_mode: bool,
    pub latency_mode_time: u8,
}

#[derive(Debug, Deserialize)]
pub struct Autoserver {
    pub enabled: bool,
    pub server_id: i8,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub language: String,
    pub update_channel: String,
    pub detect_same_system_messages: bool,
    pub message_format: String,
    pub enable_notifications: bool,
    pub enable_terminal_bell: bool,
    pub experimental_debug_mode: bool,
    pub extreme_debug_mode: bool,
    pub recv_allowed_bytes: u32,
    pub config_ver: u8,
    pub networking: Networking,
    pub autoserver: Autoserver,
    #[serde(skip)]
    pub path: String,
}

#[derive(Clone)]
pub struct ServerValuesCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct ServerValues {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub server_type: String,
    pub autologin: bool,
    pub compatibility_mode: bool,
    pub credentials: ServerValuesCredentials,
}

pub fn config_open(config_path: &str) -> String {
    fs::read_to_string(config_path).expect("Could not read config")
}

pub fn get_lang_cfg() -> String {
    include_str!("./lang.yml").to_string()
}

impl Config {
    pub fn new(config_path: String) -> Self {
        let config_yml = config_open(&config_path);
        let mut cfg: Self = from_str(&config_yml).unwrap();
        cfg.path = config_path;
        cfg
    }

    pub fn server_id(server_id: i8, config_path: &str) -> ServerValues {
        let server_id = server_id as usize;
        let config_yml = config_open(config_path);
        let config: Value = from_str(&config_yml).unwrap();

        let s_name = config["server"][server_id]["name"]
            .as_str()
            .unwrap()
            .to_string();

        let s_host = config["server"][server_id]["address"]
            .as_str()
            .unwrap()
            .to_string();

        let s_port = config["server"][server_id]["port"].as_u64().unwrap() as u16;

        let s_type = config["server"][server_id]["type"]
            .as_str()
            .unwrap()
            .to_string();

        let s_autologin = config["server"][server_id]["autologin"].as_bool().unwrap_or_default();

        let s_compatibility_mode = config["server"][server_id]["compatibility_mode"]
            .as_bool()
            .unwrap_or(false);

        let s_credentials_username = config["server"][server_id]["credentials"]["username"]
            .as_str()
            .unwrap_or("none")
            .to_string();

        let s_credentials_password = config["server"][server_id]["credentials"]["password"]
            .as_str()
            .unwrap_or("none")
            .to_string();

        ServerValues {
            name: s_name,
            address: s_host,
            port: s_port,
            server_type: s_type,
            autologin: s_autologin,
            compatibility_mode: s_compatibility_mode,
            credentials: ServerValuesCredentials {
                username: s_credentials_username,
                password: s_credentials_password,
            },
        }
    }
}
