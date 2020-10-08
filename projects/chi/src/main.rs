extern crate chi;

use std::env;

use chi::asm::*;
use std::fs::File;

fn main() {
    let mut filename = "".to_string();

    let mut is_first = true;
    for argument in env::args() {
        if is_first {
            is_first = false;
            continue;
        }
        filename = argument.to_string();
        break;
    }

    if filename.eq("") {
        panic!("there is not a file to compile.");
    }

    println!("filename: {}", &filename);

    let v = read_lines(&File::open(&filename).unwrap());

    for line in v {
        for c in line.trim().chars() {
            print!("{}", c);
        }
        println!()
    }
}
