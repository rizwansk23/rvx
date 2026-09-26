use clap::ArgMatches;
use std::path::PathBuf;

use crate::utils::{create_dir, write_file};
use clap::{Command, ValueHint, arg, value_parser};


pub fn init() -> Command {
    let init_command = Command::new("init")
        .about("initialize the command")
        .color(clap::ColorChoice::Always)
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

    create_python_file(pathname);
}

fn create_python_file(paths: &str) {
    let files = [
        ("app.py", include_str!("../../templates/app.py")),
        (".gitignore", include_str!("../../templates/.gitignore")),
        ("pyproject.toml", include_str!("../../templates/pyproject.toml")),
        ("README.md", include_str!("../../templates/README.md")),
    ];

    if paths != "." {
        create_dir::create_dir(paths);
    }

    let dir_path = PathBuf::from(paths);
    for (name, content) in files {
        write_file::write_file(dir_path.join(name).to_str().unwrap(), content.to_string());
    }
}
