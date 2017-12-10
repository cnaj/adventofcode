extern crate day06;

use std::env;
use std::io;
use std::io::BufRead;

use day06::reallocate;

fn main() {
    let input: String = match env::args().nth(1) {
        None => {
            eprintln!("Usage: {} [input_string | -]", env::args().nth(0).unwrap());
            eprintln!("If - is given as argument, read from stdin.");
            std::process::exit(0);
        },
        Some(input) => input,
    };

    if "-".eq(&input) {
        let stdin = io::stdin();
        let handle = stdin.lock();
        handle.lines().for_each(|line_res| {
            match line_res {
                Ok(line) => run_compute(&line),
                Err(err) => eprintln!("Could not read input: {}", err),
            }
        });
    } else {
        run_compute(&input)
    }
}

fn run_compute(line: &str) {
    let mut banks: Vec<u32> = vec![];
    for bank in line.split_whitespace() {
        match u32::from_str_radix(bank, 10) {
            Ok(bank) => banks.push(bank),
            Err(err) => {
                eprintln!("Not a number: {} ({})", bank, err);
                return;
            },
        }
    }

    let result = reallocate(&mut banks);
    println!("{}", result);
}