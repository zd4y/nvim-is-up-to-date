mod release;

use std::process::{self, Command};

fn get_nvim_version() -> String {
    let output = Command::new("nvim")
        .arg("--version")
        .output()
        .expect("Error running command");

    let output = String::from_utf8_lossy(&output.stdout);

    output
        .lines()
        .next()
        .expect("Error getting nvim version")
        .to_owned()
}

fn main() {
    let release = release::get_latest();
    let name = get_nvim_version();

    if name != release.name {
        process::exit(1);
    }
}
