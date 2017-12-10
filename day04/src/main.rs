extern crate day04;

use std::io;
use std::io::BufRead;

use day04::is_valid_passphrase;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut count = 0;
    for line in handle.lines() {
        match line {
            Ok(line) => {
                if is_valid_passphrase(line) {
                    count += 1;
                }
            },
            Err(err) => {
                eprintln!("Could not read input: {}", err);
                std::process::exit(1);
            }
        }
    }
    println!("{}", count);
}