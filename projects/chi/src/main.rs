extern crate chi;

use std::env;

use chi::asm::*;
use std::fs::File;

fn main() {
    let mut filename = "".to_string();
    for argument in env::args() {
        filename = argument.to_string();
        break;
    }
    let v = read_file(&File::open(&filename).unwrap());

    println!("{:?}", &v);
}
