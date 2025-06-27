use std::fs::OpenOptions;
use std::io::Write;

pub fn init_log() {
    let _ = std::fs::create_dir_all("/var/log");
    let _ = std::fs::File::create("/var/log/initron.log");
}

pub fn log(tag: &str, msg: &str) {
    let formatted = format!("[{}] {}\n", tag, msg);
    print!("{}", formatted);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/var/log/initron.log") {
        let _ = file.write_all(formatted.as_bytes());
    }
}

