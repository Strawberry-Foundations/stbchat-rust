use serde_yaml::{from_str, Value};

use stblib::strings::Strings;
use stblib::colors::*;

use crate::config::{Config, config_open};
use crate::constants;

pub fn user_server_list(string_loader: &Strings, _config: &Config, config_path: &str) -> i8 {
    println!("{BOLD}--- {CYAN}Strawberry Chat ({}){C_RESET} ---", constants::VERSION);
    println!("{GREEN}{}{C_RESET}\n", string_loader.str("Welcome"));
    println!("{BOLD}{CYAN}{UNDERLINE}{}{C_RESET}", string_loader.str("YourChatServers"));

    let config_yml = config_open(&config_path);
    let data: Value = from_str(&config_yml).unwrap();
    let server_data_length = data["server"].as_mapping().unwrap().len();

    for i in 0..server_data_length {
        println!(
            "{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}",
            i + 1,
            data["server"][i]["name"].as_str().unwrap()
        );
    }

    println!("{BOLD}{BLUE}[{}]{C_RESET}{BOLD} {}{C_RESET}\n", server_data_length + 1, string_loader.str("Custom"));

    let mut line_reader = rustyline::DefaultEditor::new().unwrap();

    let prompt = format!("{}", string_loader.str("SelChatServer"));
    let aborted = format!("{BOLD}{YELLOW}{}{C_RESET}", string_loader.str("Aborted"));

    let server_selection: u8 = match line_reader.readline(&prompt) {
        Ok(line) => match line.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!(
                    "{BOLD}{RED}{}{C_RESET}",
                    format!("{}", string_loader.str("InvalidInput"))
                );
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("{}", aborted.as_str());
            std::process::exit(1);
        }
    };

    if server_selection == (server_data_length + 1) as u8 {
        /* let host = line_reader.readline(string_loader.str("Ipaddr").as_str()).expect(aborted.as_str());
        let port: String = line_reader.readline(string_loader.str("Port").as_str()).expect(aborted.as_str()); */

        -1
    }

    else if server_selection > (server_data_length + 1) as u8 {
        eprintln!("{BOLD}{RED}{}{C_RESET}", string_loader.str("InvalidServerSelection"));
        std::process::exit(1);
    }

    else {
        (server_selection - 1) as i8
    }
}