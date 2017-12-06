use std::env;
use std::io;
use std::io::BufRead;

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
    let result = compute(&input);
    match result {
        Ok(result) => println!("{:?}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn compute(input: &str) -> Result<u32, String> {
    let chars: Vec<char> = input.chars().collect();

    let len = chars.len();
    if 0 != len % 2 {
        return Err(format!("input has odd number of characters: {}", len))
    }

    let mut sum = 0;
    for pos in 0..len {
        let cur = to_digit(chars[pos], pos)?;
        let next_pos = (pos + len / 2) % len;
        let next = to_digit(chars[next_pos], next_pos)?;
        if cur == next {
            sum += cur;
        }
    }
    Ok(sum)
}

fn to_digit(ch: char, pos: usize) -> Result<u32, String> {
    match ch.to_digit(10) {
        Some(digit) => Ok(digit),
        None => return Err(format!("unexpected input at position {}", pos)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(Ok(6), compute("1212"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(Ok(0), compute("1221"));
    }

    #[test]
    fn it_works_3() {
        assert_eq!(Ok(4), compute("123425"));
    }

    #[test]
    fn it_works_4() {
        assert_eq!(Ok(12), compute("123123"));
    }

    #[test]
    fn it_works_5() {
        assert_eq!(Ok(4), compute("12131415"));
    }

    #[test]
    fn illegal_input() {
        assert!(compute("12345").is_err());
        assert!(compute("x1234").is_err());
        assert!(compute("Hello").is_err());
        assert!(compute("1234foo").is_err());
    }

}
