use clap::{ArgMatches, Command, ValueHint, arg, value_parser};
use std::{
    env,
    path::{self, Path, PathBuf},
    process,
};

pub const VENV: &str = ".venv";

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

/// Create `.venv` if it does not exist yet.
pub fn ensure_venv() {
    if Path::new(VENV).is_dir() {
        return;
    }
    match process::Command::new("python")
        .args(["-m", "venv", VENV])
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("failed to create venv: {e}");
            process::exit(1);
        }
    }
}

/// Folder holding the venv's executables (`Scripts` on Windows, `bin` elsewhere).
pub fn venv_bin() -> PathBuf {
    Path::new(VENV).join(if cfg!(windows) { "Scripts" } else { "bin" })
}

/// "Activate" the venv for a child process: set VIRTUAL_ENV and put the
/// venv's bin dir first on PATH (what the activate scripts do).
pub fn activate_env(cmd: &mut process::Command) {
    let bin_dir = venv_bin();
    let mut paths = vec![path::absolute(&bin_dir).unwrap_or(bin_dir)];
    paths.extend(env::split_paths(&env::var_os("PATH").unwrap_or_default()));

    cmd.env("VIRTUAL_ENV", path::absolute(VENV).unwrap_or(VENV.into()))
        .env("PATH", env::join_paths(paths).unwrap())
        .env_remove("PYTHONHOME");
}

pub fn run_exec(sub_matches: &ArgMatches) {
    let pathname = sub_matches.get_one::<String>("pathname").unwrap();

    ensure_venv();

    let python = venv_bin().join(if cfg!(windows) { "python.exe" } else { "python" });
    let mut cmd = process::Command::new(python);
    cmd.arg(pathname);
    activate_env(&mut cmd);

    match cmd.status() {
        Ok(status) if status.success() => {}
        Ok(status) => process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("failed to run python: {e}");
            process::exit(1);
        }
    }
}
