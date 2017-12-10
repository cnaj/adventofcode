extern crate day05;

use std::io;
use std::io::BufRead;

use day05::execute;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut list: Vec<i32> = vec![];
    for line in handle.lines() {
        match parse_instruction(line) {
            Ok(value) => list.push(value),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    let result = execute(&mut list);
    println!("{}", result);
}

fn parse_instruction(line: io::Result<String>) -> Result<i32, String> {
    match line {
        Ok(line) => {
            match i32::from_str_radix(&line, 10) {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("Not a number: {}", line)),
            }
        },
        Err(err) => Err(format!("Could not read input: {}", err)),
    }
}