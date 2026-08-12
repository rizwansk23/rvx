use std::fs::read;
use std::char;

#[allow(unused)]
pub fn read_file(path: &str) -> String {
    let result = read(path).unwrap();

    let convert_to_string = |mut a: String, v: &u8| {
        let char = char::from(*v);
        a.push(char);
        return a;
    };

    let text = result.iter().fold(String::from(""), convert_to_string);
    
    return text;
}
