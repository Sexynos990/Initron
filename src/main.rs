mod core;
mod loader;
mod logbot;

fn main() {
    logbot::init_log();
    logbot::log("INITRON", "Init system started.");
    core::boot_sequence();
}
