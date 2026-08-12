mod commands;
mod utils;
use clap::Command;
use commands::{init,run};

fn main() {
    let matches = Command::new("vrx")
        .version("0.1.0")
        .about("VRX CLI Tool")
        .subcommand(init::init())
        .subcommand(run::run())
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => init::init_exec(sub_m),
        Some(("run", sub_m)) => run::run_exec(sub_m),
        _ => println!("Please specify a valid command or use --help"),
    }
}
