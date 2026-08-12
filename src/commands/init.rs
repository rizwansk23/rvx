use clap::ArgMatches;
use std::path::PathBuf;

use crate::utils::{create_dir, write_file};
use clap::{Command, ValueEnum, ValueHint, arg, value_parser};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Language {
    Python,
    React,
    Flask,
}

impl Language {
    fn run(&self, path: &str) {
        match self {
            Language::Python => create_python_file(path),
            Language::React => println!("React file created"),
            Language::Flask => println!("Flask file created"),
        }
    }
}

pub fn init() -> Command {
    let init_command = Command::new("init")
        .about("initialize the command")
        .color(clap::ColorChoice::Always)
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
        );

    return init_command;
}

pub fn init_exec(sub_matches: &ArgMatches) {
    
    let pathname = sub_matches.get_one::<String>("pathname").unwrap();

    if let Some(lang) = sub_matches.get_one::<Language>("language") {
        lang.run(pathname);
        println!("file created successfully")
    }
}

fn create_python_file(paths: &str) {
    let app = include_str!("../../templates/python/app.py");
    let gitignore = include_str!("../../templates/python/.gitignore");
    let pyproject = include_str!("../../templates/python/pyproject.toml");
    let readme = include_str!("../../templates/python/README.md");

    if paths == "." {
        write_file::write_file("app.py", app.to_string());
        write_file::write_file(".gitignore", gitignore.to_string());
        write_file::write_file("pyproject.toml", pyproject.to_string());
        write_file::write_file("README.md", readme.to_string());
    } else {
        let dir_path = PathBuf::from(paths);

        create_dir::create_dir(paths);

        let app_path = dir_path.join("app.py");
        let gitignore_path = dir_path.join(".gitignore");
        let pyproject_path = dir_path.join("pyproject.toml");
        let readme_path = dir_path.join("README.md");

        write_file::write_file(&app_path.to_str().unwrap(), app.to_string());
        write_file::write_file(&gitignore_path.to_str().unwrap(), gitignore.to_string());
        write_file::write_file(&pyproject_path.to_str().unwrap(), pyproject.to_string());
        write_file::write_file(&readme_path.to_str().unwrap(), readme.to_string());
    }
}
