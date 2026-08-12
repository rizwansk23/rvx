use std::fs::create_dir_all;

#[allow(unused)]
pub fn create_dir(dir_path:&str) {
    let created = create_dir_all(&format!("./{dir_path}"));
    if created.is_ok() {
        print!("directory created.")
    } else {
        print!("oops.. something wrong")
    }
}
