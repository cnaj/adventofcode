use State::*;

enum State {
    Start,
    Group,
    Garbage,
    Escape,
}

pub fn get_score(input: &str) -> usize {
    let mut analyzer = StreamAnalyzer::new();
    input.chars()
        .for_each(|ch| analyzer.next_char(ch));
    analyzer.get_score()
}

pub struct StreamAnalyzer {
    state: State,
    level: usize,
    score: usize,
}

impl StreamAnalyzer {
    pub fn new() -> StreamAnalyzer {
        StreamAnalyzer {
            state: Start,
            level: 0,
            score: 0,
        }
    }

    pub fn next_char(&mut self, ch: char) {
        match self.state {
            Start => {
                match ch {
                    '{' => {
                        self.state = Group;
                        self.level = 1;
                        self.score += 1;
                    },
                    '<' => {
                        self.state = Garbage;
                    }
                    _ => (),
                };
            },

            Group => {
                match ch {
                    '{' => {
                        self.level += 1;
                        self.score += self.level;
                    },
                    '}' => {
                        self.level -= 1;
                        if 0 == self.level {
                            self.state = Start;
                        }
                    },
                    '<' => {
                        self.state = Garbage;
                    },
                    _ => (),
                };
            },

            Garbage => {
                match ch {
                    '>' => {
                        self.state = if self.level == 0 {
                            Start
                        } else {
                            Group
                        };
                    },
                    '!' => {
                        self.state = Escape;
                    },
                    _ => (),
                };
            },

            Escape => {
                self.state = Garbage;
            },
        };
    }

    pub fn get_score(&self) -> usize {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, get_score("{}"));
        assert_eq!(6, get_score("{{{}}}"));
        assert_eq!(5, get_score("{{},{}}"));
        assert_eq!(16, get_score("{{{},{},{{}}}}"));
        assert_eq!(1, get_score("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, get_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, get_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, get_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}
