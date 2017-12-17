#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;

use regex::Regex;

pub struct ProgramExecutor {
    registers: HashMap<String, i32>,
    max_value: Option<i32>,
}

impl ProgramExecutor {

    pub fn new() -> ProgramExecutor {
        ProgramExecutor {
            registers: HashMap::new(),
            max_value: None,
        }
    }

    pub fn add_instruction(&mut self, line: &str) -> Result<(), String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(\w+) (inc|dec) (-?\d+) if (\w+) (<|<=|>|>=|==|!=) (-?\d+)"
            ).unwrap();
        }

        RE.captures(line)
            .map_or_else(
                || Err(format!("invalid input line: {}", line)),
                |cpt| {
                    if self.test(&cpt[4], &cpt[5],
                                 i32::from_str_radix(&cpt[6], 10).unwrap()) {
                        self.execute(&cpt[1], &cpt[2],
                                     i32::from_str_radix(&cpt[3], 10).unwrap());
                    }
                    Ok(())
                },
            )
    }

    pub fn get_largest_register_value(&self) -> Option<i32> {
        self.registers.values()
            .max()
            .map(|i| *i)
    }

    pub fn get_max_value(&self) -> Option<i32> {
        self.max_value
    }

    fn test(&mut self, reg: &str, cond: &str, value: i32) -> bool {
        let reg_value = *self.registers.get(reg).unwrap_or(&0);
        match cond {
            "<" => reg_value < value,
            "<=" => reg_value <= value,
            ">" => reg_value > value,
            ">=" => reg_value >= value,
            "==" => reg_value == value,
            "!=" => reg_value != value,
            _ => panic!("unexpected instruction {}", cond),
        }
    }

    fn execute(&mut self, reg: &str, op: &str, amount: i32) {
        let reg_value = *self.registers.get(reg).unwrap_or(&0);
        let new_value = match op {
            "inc" => reg_value + amount,
            "dec" => reg_value - amount,
            _ => panic!("unexpected instruction {}", op),
        };
        self.registers.insert(reg.to_owned(), new_value);

        let max = match self.max_value {
            Some(v) => v.max(new_value),
            None => new_value,
        };
        self.max_value = Some(max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = [
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ];

        let mut executor = ProgramExecutor::new();
        lines.iter()
            .for_each(|line| executor.add_instruction(&line).unwrap());

        assert_eq!(Some(10), executor.get_max_value());
    }
}
