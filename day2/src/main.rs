use std::cmp;
use std::fmt::Display;
use std::io;
use std::io::BufRead;
use std::ops::Deref;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let lines = handle.lines();
    match compute(lines) {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("Error: {}", err),
    };
}

fn compute<'a, I, T, E>(input: I) -> Result<i32, String>
    where
        I: IntoIterator<Item = Result<T, E>>,
        T: Deref<Target = str>,
        E: Display
{
    let lines = input.into_iter();

    let mut sum = 0;
    for line_result in lines {

        let line = match line_result {
            Ok(line) => line,
            Err(err) => return Err(format!("Error while reading line: {}", err)),
        };

        let mut bounds = Option::None;

        for cell in line.split_whitespace() {
            match i32::from_str_radix(cell, 10) {
                Ok(value) => {
                    bounds = match bounds {
                        Some((min, max)) => Some((cmp::min(min, value),
                                                  cmp::max(max, value))),
                        None => Some((value, value)),
                    };
                },
                Err(_) => return Err(format!("Not a number: {}", cell)),
            };
        }

        if let Some((min, max)) = bounds {
            sum += max - min;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input = "5  1  9  5\n\
                           7  5  3   \n\
                           2  4  6  8";
        assert_eq!(Ok(18), compute(input.lines().map(|line| wrap(line))));
    }

    fn wrap(input: &str) -> Result<&str, String> {
        Ok(input)
    }

}
