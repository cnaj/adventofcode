#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

use BalanceResult::*;

#[derive(Eq, PartialEq, Debug)]
pub enum BalanceResult<'a> {
    Balanced(usize),
    Unbalanced(&'a str, usize),
}

pub struct ProgramAnalyzer {
    programs: HashMap<String, (usize, Vec<String>)>,
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
                let weight = usize::from_str_radix(&capture[2], 10).unwrap();

                let others = capture.get(3)
                    .map_or_else(
                        || vec![],
                        |other_cpt| {
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
            .map(|elem| self.find_parent_rec(elem))
    }

    fn find_parent_rec<'a>(&'a self, elem: &'a str) -> &'a str {
        match self.parents.get(elem) {
            Some(parent) => self.find_parent_rec(parent),
            None => elem,
        }
    }

    pub fn balance<'a>(&'a self) -> Option<BalanceResult<'a>> {
        self.find_bottom()
            .map(|bottom| self.balance_rec(bottom))
    }

    fn balance_rec<'a>(&'a self, elem: &'a str) -> BalanceResult<'a> {
        let (weight, ref others) = self.programs[elem];
        if others.is_empty() {
            return Balanced(weight);
        }

        let mut sum_weight: usize = 0;
        let mut weights: Vec<(&'a str, usize)> = vec![];
        let mut weight_freq: HashMap<usize, usize> = HashMap::new();

        for other in others {
            match self.balance_rec(other) {
                Balanced(w) => {
                    sum_weight += w;
                    weights.push((other, w));
                    let count = match weight_freq.get(&w) {
                        Some(f) => f + 1,
                        None => 1,
                    };
                    weight_freq.insert(w, count);
                },
                Unbalanced(n, c) => return Unbalanced(n, c),
            }
        }

        if 1 >= weight_freq.len() {
            return Balanced(weight + sum_weight);
        }

        let odd_weight = *weight_freq.iter()
            .min_by_key(|&(_, f)| f)
            .map(|(w, _)| w)
            .unwrap();
        let normal_weight = *weight_freq.iter()
            .max_by_key(|&(_, f)| f)
            .map(|(w, _)| w)
            .unwrap();

        let unbal_name = weights.iter()
            .find(|&&(_, w)| w == odd_weight)
            .map(|&(name, _)| name)
            .unwrap();
        let unbal_weight = self.programs[unbal_name].0;

        let corr_weight = if odd_weight > normal_weight {
            unbal_weight - (odd_weight - normal_weight)
        } else {
            unbal_weight + (normal_weight - odd_weight)
        };

        Unbalanced(unbal_name, corr_weight)
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

        assert_eq!(Some(Unbalanced("ugml", 60)), analyzer.balance());
    }
}
