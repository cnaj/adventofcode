use std::collections::hash_map::HashMap;
use std::ops::Deref;

pub fn is_valid_passphrase<T: Deref<Target = str>>(input: T) -> bool {
    let phrase: &str = input.deref();

    let mut words = HashMap::new();

    for word in phrase.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        *words.entry(chars).or_insert(0) += 1;
    }

    !words.is_empty() && words.values().find(|c| **c > 1).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_valid_passphrase("abcde fghij"));
        assert_eq!(false, is_valid_passphrase("abcde xyz ecdab"));
        assert_eq!(true, is_valid_passphrase("a ab abc abd abf abj"));
        assert_eq!(true, is_valid_passphrase("iiii oiii ooii oooi oooo"));
        assert_eq!(false, is_valid_passphrase("oiii ioii iioi iiio"));
    }
}
