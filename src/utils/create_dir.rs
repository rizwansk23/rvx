use std::fs::create_dir_all;

#[allow(unused)]
pub fn create_dir() {
    let created = create_dir_all("./data");
    if created.is_ok() {
        print!("directory created.")
    } else {
        print!("oops.. something wrong")
    }
}
