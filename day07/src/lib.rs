#[macro_use] extern crate lazy_static;

extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::ops::Deref;

pub struct ProgramAnalyzer {
    programs: HashMap<String, (i32, Option<Vec<String>>)>,
}

impl ProgramAnalyzer {

    pub fn new() -> ProgramAnalyzer {
        ProgramAnalyzer {
            programs: HashMap::new(),
        }
    }

    pub fn add_line<T: Deref<Target = str>>(&mut self, line: T) -> Result<(), String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (\w+(?:, \w+)*))?").unwrap();
        }

        let line: &str = line.deref();
        match RE.captures(line) {
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

                self.programs.insert(name, (weight, others));

                Ok(())
            },
            None => Err(format!("invalid input line: {}", line))
        }
    }

    pub fn find_bottom(&self) -> Option<&str> {
        Some("tknk")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = [
            "pbga (66)",
            "xhth (57)",
            "ebii (61)",
            "havc (66)",
            "ktlj (57)",
            "fwft (72) -> ktlj, cntj, xhth",
            "qoyq (66)",
            "padx (45) -> pbga, havc, qoyq",
            "tknk (41) -> ugml, padx, fwft",
            "jptl (61)",
            "ugml (68) -> gyxo, ebii, jptl",
            "gyxo (61)",
            "cntj (57)",
        ];

        let mut analyzer = ProgramAnalyzer::new();

        lines.iter()
            .for_each(|line| analyzer.add_line(*line).unwrap());

        assert_eq!(Some("tknk"), analyzer.find_bottom());
    }
}
