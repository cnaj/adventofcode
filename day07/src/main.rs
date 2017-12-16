extern crate day07;

use std::io;
use std::io::BufRead;

use day07::ProgramAnalyzer;
use day07::BalanceResult::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    match compute(handle) {
        Ok(result) => {
            if let Some((name, weight)) = result {
                println!("New weight for {}: {}", name, weight);
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        },
    }
}

fn compute(handle: io::StdinLock) -> Result<Option<(String, usize)>, String> {
    let mut analyzer = ProgramAnalyzer::new();

    let lines = handle.lines();
    for line_result in lines {
        match line_result {
            Ok(line) => analyzer.add_line(&line)?,
            Err(err) => return Err(format!("Could not read line: {}", err)),
        }
    }

    let result =
        analyzer.balance()
            .and_then(|result| {
                match result {
                    Balanced(_) => None,
                    Unbalanced(n, w) => Some((n.to_string(), w)),
                }
            });
    Ok(result)
}