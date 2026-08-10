use crate::utils::{read_file, write_file};
use clap::{Command, ValueEnum, ValueHint, arg, command, value_parser};
// use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Language {
    Python,
    React,
    Flask,
}

impl Language {
    fn run(&self) {
        match self {
            Language::Python => create_python_file("hello"),
            Language::React => println!("React file created"),
            Language::Flask => println!("Flask file created"),
        }
    }
}

pub fn init() {
    let init_command = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("initialize the command")
                .arg(
                    arg!([language])
                        .value_parser(value_parser!(Language))
                        .required(true)
                        .help("help to make template"),
                )
                .arg(
                    arg!([pathname])
                        .value_hint(ValueHint::FilePath)
                        .value_parser(value_parser!(String))
                        .required(true),
                ),
        )
        .get_matches();

    // let mut init_method: HashMap<String, fn()> = HashMap::new();

    // init_method.insert("python".to_string(), create_python_file);

    // if let Some(("init", sub_matches)) = init_command.subcommand() {
    //     if let Some(lang) = sub_matches.get_one::<String>("language") {
    //         let key = lang.to_lowercase();
    //         if let Some(action) = init_method.get(&key) {
    //             action();
    //             print!("file created")
    //         } else {
    //             println!("Unsupported language: {}", lang);
    //         }
    //     }
    // }

    if let Some(("init", sub_matches)) = init_command.subcommand() {
        if let Some(lang) = sub_matches.get_one::<Language>("language") {
            lang.run();
            println!("file created successfully")
        }
    };

    match init_command.subcommand() {
        Some(("init", sub_matches)) => match sub_matches.get_one::<String>("pathname") {
            Some(paths) if paths == "." => print!("file paths ."),

            Some(paths) => print!("craeted {}",paths),

            _ => ()
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };

}

fn create_python_file(paths:&str) {
    let path = "./src/head.txt";

    let text = read_file::read_file(paths);

    write_file::write_file("./src/script.py", text);
}
