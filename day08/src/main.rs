extern crate day08;

use std::io;
use std::io::BufRead;

use day08::ProgramExecutor;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    match compute(handle) {
        Ok(result) => {
            if let Some(result) = result {
                println!("{}", result)
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        },
    }
}

fn compute(handle: io::StdinLock) -> Result<Option<i32>, String> {
    let mut executor = ProgramExecutor::new();

    let lines = handle.lines();
    for line_result in lines {
        match line_result {
            Ok(line) => executor.add_instruction(&line)?,
            Err(err) => return Err(format!("Could not read line: {}", err)),
        };
    }

    Ok(executor.get_max_value())
}
