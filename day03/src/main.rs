extern crate day03;

use std::env;
use std::io;
use std::io::BufRead;

use day03::compute;

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

fn run_compute(input: &str) {
    match u32::from_str_radix(input, 10) {
        Ok(value) => println!("{}", compute(value)),
        Err(_) => eprintln!("Not a number: {}", input),
    }
}
