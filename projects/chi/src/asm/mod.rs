use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;

pub fn read_all_bytes(file: &File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut v = Vec::new();

    let result = reader.read_to_end(&mut v);
    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err());
    }
    v
}

pub fn read_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut res = Vec::new();

    for l in reader.lines() {
        if l.is_ok() {
            res.push(l.unwrap())
        } else {
            println!("{}", l.unwrap_err());
            break;
        }
    }
    res
}

pub fn read_to_string(file: &File) -> String {
    let mut reader = BufReader::new(file);
    let mut s = "".to_string();

    let result =reader.read_to_string(&mut s);

    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err());
    }
    s
}