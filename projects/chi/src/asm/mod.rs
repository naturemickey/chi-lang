use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;

pub fn read_bytes(file: &File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut v = Vec::new();

    let result = reader.read_to_end(&mut v);
    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err());
    }
    v
}

pub fn read_lines(file: &File) -> Vec<String> {
    let mut reader = BufReader::new(file);
    let mut line= "".to_string();
    let mut res = Vec::new();
    loop {
        let result = reader.read_line(&mut line);
        if result.is_ok() {
            res.push(line.to_string())
        } else {
            println!("{}", result.unwrap_err());
            break;
        }
    }
    return res;
}