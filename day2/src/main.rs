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

        let values: Vec<i32> = values_from_line(line.deref())?;

        if let Some(line_result) = first_divisible_result(&values) {
            sum += line_result;
        }
    }
    Ok(sum)
}

fn values_from_line(line: &str) -> Result<Vec<i32>, String> {
    let mut values: Vec<i32> = vec![];
    for cell in line.split_whitespace() {
        match i32::from_str_radix(cell, 10) {
            Ok(value) => values.push(value),
            Err(_) => return Err(format!("Not a number: {}", cell)),
        };
    }
    Ok(values)
}

fn first_divisible_result<T: Deref<Target = [i32]>>(values: &T) -> Option<i32> {
    let mut it_i = values.iter();
    while let Some(i) = it_i.next() {

        let mut it_j = it_i.clone();
        while let Some(j) = it_j.next() {
            let (i, j) = if i < j {
                (i, j)
            } else {
                (j, i)
            };

            if j % i == 0 {
                return Some(j / i)
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let input = "5  9  2  8\n\
                           9  4  7  3\n\
                           3  8  6  5";
        assert_eq!(Ok(9), compute(input.lines().map(|line| wrap(line))));
    }

    fn wrap(input: &str) -> Result<&str, String> {
        Ok(input)
    }

}
