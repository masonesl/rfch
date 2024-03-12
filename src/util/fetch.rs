pub mod fetch {
    use std::env;
    use sysinfo::System;

    pub fn osname() -> String {
        match System::name() {
            Some(name) => name,
            None => "UNKNOWN".to_string(),
        }
    }

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
}
