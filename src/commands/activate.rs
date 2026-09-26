use clap::Command;
use std::{env, process};

use super::run::{VENV, activate_env, ensure_venv};

pub fn activate() -> Command {
    Command::new("activate")
        .about("open a shell with .venv activated (type `exit` to leave)")
        .color(clap::ColorChoice::Always)
}

pub fn activate_exec() {
    ensure_venv();

    let mut cmd = if cfg!(windows) {
        let mut c = process::Command::new("powershell");
        c.args([
            "-NoLogo",
            "-NoExit",
            "-Command",
            &format!("function global:prompt {{ \"({VENV}) PS $($PWD.Path)> \" }}"),
        ]);
        c
    } else {
        process::Command::new(env::var("SHELL").unwrap_or("sh".into()))
    };
    activate_env(&mut cmd);

    println!("{VENV} activated. Type `exit` to leave.");
    if let Err(e) = cmd.status() {
        eprintln!("failed to start shell: {e}");
        process::exit(1);
    }
}
