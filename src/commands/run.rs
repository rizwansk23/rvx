use clap::{ArgMatches, Command, ValueHint, arg, value_parser};
use std::process;

pub fn run() -> Command {
    let run_command = Command::new("run")
        .about("run a python file")
        .color(clap::ColorChoice::Always)
        .arg(
            arg!([pathname])
                .value_hint(ValueHint::FilePath)
                .value_parser(value_parser!(String))
                .default_value("app.py"),
        );

    return run_command;
}

pub fn run_exec(sub_matches: &ArgMatches) {
    let pathname = sub_matches.get_one::<String>("pathname").unwrap();

    match process::Command::new("python").arg(pathname).status() {
        Ok(status) if status.success() => {}
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("failed to run python: {e}");
            process::exit(1);
        }
    }
}
