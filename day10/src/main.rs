extern crate day10;

use std::io;
use std::io::BufRead;

use day10::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line_result in handle.lines() {
        match line_result {
            Ok(line) => {
                compute(&line);
            },
            Err(err) => {
                eprintln!("Could not read line: {}", err);
                std::process::exit(1);
            },
        };
    }
}

fn compute(line: &str) {
    let mut list = [0u8; 256];
    let result = knot_hash(&mut list, line);
    println!("{}", result);
}