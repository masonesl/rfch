pub mod fetch {
    use std::{env, path};
    use sysinfo::System;

    macro_rules! unknown {
        () => {
            "unknown".to_string()
        };
    }

    pub fn username() -> String {
        return env::var("USER")
            .unwrap_or(unknown!());
    }

    pub fn hostname() -> String {
        return System::host_name()
            .unwrap_or(unknown!());
    }

    pub fn osname() -> String {
        return System::name()
            .unwrap_or(unknown!());
    }

    pub fn kernel() -> String {
        return System::kernel_version()
            .unwrap_or(unknown!());
    }

    pub fn editor() -> String {
        return env::var("EDITOR")
            .unwrap_or(unknown!());
    }

    pub fn terminal() -> String {
        return env::var("TERM")
            .unwrap_or(unknown!());
    }

    pub fn desktop() -> String {
        return env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| env::var("DESKTOP_SESSION")
                            .unwrap_or(unknown!()));
    }

    pub fn shell() -> String {
        // attempt to read the SHELL environment variable
        return match env::var("SHELL") {
            Ok(shell_path) => {
                // convert the shell binary path to Path object and grab the actual binary name
                match path::Path::new(&shell_path).file_name() {
                    Some(shell_name) => {
                        shell_name.to_str()
                            .unwrap_or("unknown")
                            .to_string()
                    },
                    None => unknown!(),
                }
            },
            Err(_) => unknown!(),
        };
    }

    pub fn cpu(system_info: &System) -> String {
        // get the cpu name from the first core
        return system_info
            .cpus()[0]
            .brand()
            .to_string();
    }
}
