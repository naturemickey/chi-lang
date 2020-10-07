use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;
use std::error::Error;

pub fn read_file(file: &File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut v = Vec::new();

    let result = reader.read_to_end(&mut v);
    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err().description());
    }
    v
}