use nix::mount::{mount, MsFlags};
use nix::unistd::{getpid, fork, ForkResult};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;
use crate::loader;
use crate::logbot;

fn is_mounted(mountpoint: &str) -> bool {
    if let Ok(file) = File::open("/proc/mounts") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some(mnt) = line.split_whitespace().nth(1) {
                if mnt == mountpoint {
                    return true;
                }
            }
        }
    }
    false
}

pub fn mount_essential_fs() {
    logbot::log("INITRON", "Mounting essential filesystems...");

    for dir in ["/proc", "/sys", "/dev", "/dev/pts"] {
        if let Err(e) = fs::create_dir_all(dir) {
            logbot::log("INITRON", &format!("Failed to create {}: {}", dir, e));
        }
    }

    if getpid().as_raw() == 1 {
        if !is_mounted("/proc") {
            if let Err(e) = mount(Some("proc"), "/proc", Some("proc"), MsFlags::empty(), None::<&str>) {
                logbot::log("INITRON", &format!("Failed to mount /proc: {}", e));
            }
        }
        if !is_mounted("/sys") {
            if let Err(e) = mount(Some("sysfs"), "/sys", Some("sysfs"), MsFlags::empty(), None::<&str>) {
                logbot::log("INITRON", &format!("Failed to mount /sys: {}", e));
            }
        }
        if !is_mounted("/dev") {
            if let Err(e) = mount(Some("devtmpfs"), "/dev", Some("devtmpfs"), MsFlags::empty(), None::<&str>) {
                logbot::log("INITRON", &format!("Failed to mount /dev: {}", e));
            }
        }
        if !is_mounted("/dev/pts") {
            if let Err(e) = mount(Some("devpts"), "/dev/pts", Some("devpts"), MsFlags::empty(), None::<&str>) {
                logbot::log("INITRON", &format!("Failed to mount /dev/pts: {}", e));
            }
        }
    } else {
        logbot::log("INITRON", "Not PID 1, skipping essential mounts.");
    }
}

fn spawn_getty() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let err = Command::new("/sbin/agetty")
                .arg("--noclear")
                .arg("tty1")
                .arg("linux")
                .status();

            if let Err(e) = err {
                logbot::log("INITRON", &format!("Failed to exec agetty: {}", e));
            }

            std::process::exit(1);
        }
        Ok(ForkResult::Parent { child }) => {
            logbot::log("INITRON", &format!("Spawned agetty on tty1 with PID {}", child));
        }
        Err(e) => {
            logbot::log("INITRON", &format!("Failed to fork agetty: {}", e));
        }
    }
}

fn reap_zombies() {
    use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
    loop {
        match waitpid(None, Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, code)) => {
                logbot::log("INITRON", &format!("PID {} exited with code {}", pid, code));
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                logbot::log("INITRON", &format!("PID {} killed by signal {:?}", pid, signal));
            }
            Ok(WaitStatus::StillAlive) => {
                break;
            }
            Ok(other) => {
                logbot::log("INITRON", &format!("Unhandled wait status: {:?}", other));
            }
            Err(e) => {
                logbot::log("INITRON", &format!("waitpid failed: {}", e));
                break;
            }
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn fake_systemd_symlinks() {
    let _ = fs::create_dir_all("/run/systemd/system");
    let _ = std::os::unix::fs::symlink("/initron", "/sbin/init");
    logbot::log("INITRON", "Created fake systemd symlinks.");
}

pub fn boot_sequence() {
    fake_systemd_symlinks();
    mount_essential_fs();
    loader::load_services("/etc/initron.d");
    spawn_getty();

    loop {
        reap_zombies();
    }
}

