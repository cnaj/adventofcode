extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::ops::Deref;

pub fn find_bottom<T: Deref<Target = str>>(input: &[T]) -> Result<String, String> {
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (\w+(?:, \w+)*))?").unwrap();

    let mut programs = HashMap::new();

    for line in input {
        let line: &str = line.deref();

        match re.captures(line) {
            Some(capture) => {
                let name = capture[1].to_string();
                let weight = i32::from_str_radix(&capture[2], 10).unwrap();

                let others: Option<Vec<String>> = match capture.get(3) {
                    Some(other_cpt) => {
                        Some(other_cpt.as_str()
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.to_string())
                            .collect())
                    },
                    None => None,
                };
                programs.insert(name, (weight, others));
            },
            None => return Err(format!("invalid input line: {}", line)),
        };
    }

    Ok("tknk".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = [
            "pbga (66)",
            "xhth (57)",
            "fwft (72) -> ktlj, cntj, xhth",
        ];
        assert_eq!(Ok("tknk".to_string()), find_bottom(&lines));
    }
}
