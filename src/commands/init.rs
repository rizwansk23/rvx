use crate::utils::{read_file, write_file};

pub fn init() {
    println!("this is init file");
    create_python_file();
}

fn create_python_file() {
    let path = "./src/head.txt";

    let text = read_file::read_file(path);

    write_file::write_file("./src/script.py", text);
}
