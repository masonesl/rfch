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
}
