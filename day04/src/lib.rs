use std::collections::hash_map::HashMap;
use std::ops::Deref;

pub fn is_valid_passphrase<T: Deref<Target = str>>(input: T) -> bool {
    let phrase: &str = input.deref();

    let mut words = HashMap::new();

    for word in phrase.split_whitespace() {
        *words.entry(word).or_insert(0) += 1;
    }

    !words.is_empty() && words.values().find(|c| **c > 1).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, is_valid_passphrase(""));
        assert_eq!(true, is_valid_passphrase("aa bb cc dd ee"));
        assert_eq!(false, is_valid_passphrase("aa bb cc dd aa"));
        assert_eq!(true, is_valid_passphrase("aa bb cc dd aaa"));
    }
}
