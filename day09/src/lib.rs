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

pub fn count_garbage(input: &str) -> usize {
    let mut analyzer = StreamAnalyzer::new();
    input.chars()
        .for_each(|ch| analyzer.next_char(ch));
    analyzer.count_garbage()
}

pub struct StreamAnalyzer {
    state: State,
    level: usize,
    score: usize,
    garbage_count: usize,
}

impl StreamAnalyzer {
    pub fn new() -> StreamAnalyzer {
        StreamAnalyzer {
            state: Start,
            level: 0,
            score: 0,
            garbage_count: 0,
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
                    _ => {
                        self.garbage_count += 1;
                    },
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

    pub fn count_garbage(&self) -> usize {
        self.garbage_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score() {
        assert_eq!(1, get_score("{}"));
        assert_eq!(6, get_score("{{{}}}"));
        assert_eq!(5, get_score("{{},{}}"));
        assert_eq!(16, get_score("{{{},{},{{}}}}"));
        assert_eq!(1, get_score("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, get_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, get_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, get_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }

    #[test]
    fn test_count_garbage() {
        assert_eq!(0, count_garbage("<>"));
        assert_eq!(17, count_garbage("<random characters>"));
        assert_eq!(3, count_garbage("<<<<>"));
        assert_eq!(2, count_garbage("<{!>}>"));
        assert_eq!(0, count_garbage("<!!>"));
        assert_eq!(0, count_garbage("<!!!>>"));
        assert_eq!(10, count_garbage(r#"<{o"i!a,<{i<a>"#));
    }
}
