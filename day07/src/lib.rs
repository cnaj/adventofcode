#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

pub struct ProgramAnalyzer {
    programs: HashMap<String, (i32, Option<Vec<String>>)>,
    parents: HashMap<String, String>,
}

impl ProgramAnalyzer {

    pub fn new() -> ProgramAnalyzer {
        ProgramAnalyzer {
            programs: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) -> Result<(), String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (\w+(?:, \w+)*))?").unwrap();
        }

        match RE.captures(line) {
            Some(capture) => {
                let name = capture[1].to_string();
                let weight = i32::from_str_radix(&capture[2], 10).unwrap();

                let others = capture.get(3)
                    .map(|other_cpt| {
                        other_cpt.as_str()
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.to_string())
                            .map(|s| {
                                self.parents.insert(s.to_owned(), name.to_owned());
                                s
                            })
                            .collect()
                    });

                self.programs.insert(name, (weight, others));
                Ok(())
            },
            None => Err(format!("invalid input line: {}", line))
        }
    }

    pub fn find_bottom(&self) -> Option<&str> {
        self.parents.keys().next()
            .and_then(|elem| self.find_parent_rec(elem))
    }

    fn find_parent_rec<'a>(&'a self, elem: &'a str) -> Option<&'a str> {
        match self.parents.get(elem) {
            Some(parent) => self.find_parent_rec(parent),
            None => Some(elem),
        }
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
            .for_each(|line| analyzer.add_line(&line).unwrap());

        assert_eq!(Some("tknk"), analyzer.find_bottom());
    }
}
