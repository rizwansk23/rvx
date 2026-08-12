use clap::{ArgMatches, Command, arg, value_parser};

pub fn run() -> Command {
    let run_command = Command::new("run")
        .about("initialize the command")
        .color(clap::ColorChoice::Always)
        .arg(
            arg!([pathname])
                .value_parser(value_parser!(String))
                .required(true),
        );

    return run_command;
}

pub fn run_exec(sub_matches:&ArgMatches){
    let pathname = sub_matches.get_one::<String>("pathname").unwrap();

    println!("fulename {}", pathname)
}