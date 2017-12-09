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
    // operating on bytes because we need digits '0'..'9' only

    let len = input.len();

    let mut next_it = input.bytes().enumerate().cycle().skip(len / 2);
    let mut sum = 0;

    for (pos, b) in input.bytes().enumerate() {
        let cur = to_digit(b, pos)?;

        let (pos_next, b_next) = next_it.next().unwrap();
        let next = to_digit(b_next, pos_next)?;

        if cur == next {
            sum += cur;
        }
    }
    Ok(sum)
}

fn to_digit(b: u8, pos: usize) -> Result<u32, String> {
    if b < b'0' || b > b'9' {
        return Err(format!("unexpected input at character position {}", pos));
    }
    Ok((b - b'0') as u32)
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
        assert!(compute("x1234").is_err());
        assert!(compute("Hello").is_err());
        assert!(compute("1234foo").is_err());
    }

}
