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
                println!("{}", hash_str(&line));
            },
            Err(err) => {
                eprintln!("Could not read line: {}", err);
                std::process::exit(1);
            },
        };
    }
}
