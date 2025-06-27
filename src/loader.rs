use std::fs;
use std::process::Command;
use crate::logbot;
use std::os::unix::fs::PermissionsExt;

pub fn load_services(dir: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                let file_str = path.to_string_lossy().to_string();
                let name = path.file_name().unwrap_or_default().to_string_lossy();

                if path.is_file() && path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                    logbot::log("INITRON", &format!("Starting service: {}", name));
                    match Command::new(&file_str).arg("start").spawn() {
                        Ok(child) => {
                            logbot::log("INITRON", &format!("Service {} started with PID {}", name, child.id()));
                        }
                        Err(e) => {
                            logbot::log("INITRON", &format!("Failed to start service {}: {}", name, e));
                        }
                    }
                }
            }
        }
    } else {
        logbot::log("INITRON", &format!("Could not read service dir: {}", dir));
    }
}

