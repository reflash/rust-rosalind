use std::fs;

pub fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath)
        .expect("Something went wrong reading the file")
}