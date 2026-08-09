use std::fs::write;

pub fn write_file(path:&str , text : String) {
    _ = write(path, text);
}
