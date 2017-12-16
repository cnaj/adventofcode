extern crate day07;

use std::io;
use std::io::BufRead;

use day07::ProgramAnalyzer;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    match compute(handle) {
        Ok(bottom) => {
            if let Some(bottom) = bottom {
                println!("Bottom program is \"{}\"", bottom);
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        },
    }
}

fn compute(handle: io::StdinLock) -> Result<Option<String>, String> {
    let mut analyzer = ProgramAnalyzer::new();

    let lines = handle.lines();
    for line_result in lines {
        match line_result {
            Ok(line) => analyzer.add_line(line)?,
            Err(err) => return Err(format!("Could not read line: {}", err)),
        }
    }

    let result =
        analyzer.find_bottom()
            .map(|s| s.to_string());
    Ok(result)
}