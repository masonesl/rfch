pub mod fetch {
    use std::{env, path};
    use sysinfo::System;

    pub fn username() -> String {
        match env::var("USER") {
            Ok(name) => name,
            Err(_) => "UNKNOWN".to_string(),
        }
    }

    pub fn hostname() -> String {
        match System::host_name() {
            Some(name) => name,
            None => "UNKNOWN".to_string(),
        }
    }

    pub fn osname() -> String {
        match System::name() {
            Some(name) => name,
            None => "UNKNOWN".to_string(),
        }
    }

    pub fn kernel() -> String {
        match System::kernel_version() {
            Some(name) => name,
            None => "UNKNOWN".to_string(),
        }
    }

    pub fn editor() -> String {
        match env::var("EDITOR") {
            Ok(name) => name,
            Err(_) => "UNKNOWN".to_string(),
        }
    }

    pub fn terminal() -> String {
        match env::var("TERM") {
            Ok(name) => name,
            Err(_) => "UNKNOWN".to_string(),
        }
    }

    pub fn desktop() -> String {
        match env::var("XDG_CURRENT_DESKTOP") {
            Ok(name) => name,
            Err(_) => {
                match env::var("DESKTOP_SESSION") {
                    Ok(name) => name,
                    Err(_) => "UNKNOWN".to_string(),
                }
            }
        }
    }

    pub fn shell() -> String {
        // attempt to read the SHELL environment variable
        match env::var("SHELL") {
            Ok(shell_path) => {
                // convert the shell binary path to Path object and grab the actual binary name
                match path::Path::new(&shell_path).file_name() {
                    Some(shell_name) => match shell_name.to_str() {
                        Some(shell_name_str) => shell_name_str.to_string(),
                        None => "UNKNOWN".to_string()
                    },
                    None => "UNKNOWN".to_string(),
                }
            },
            Err(_) => "UNKNOWN".to_string(),
        }
    }
}
