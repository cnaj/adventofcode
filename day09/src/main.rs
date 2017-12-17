extern crate day09;

use std::io;
use std::io::BufRead;

use day09::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut analyzer = StreamAnalyzer::new();

    // reading lines because Stdin.chars() isn't stabilized yet

    for line_result in handle.lines() {
        match line_result {
            Ok(line) => {
                line.chars()
                    .for_each(|ch| analyzer.next_char(ch));
            },
            Err(err) => {
                eprintln!("Could not read line: {}", err);
                std::process::exit(1);
            },
        };
    }

    println!("{}", analyzer.count_garbage());
}
