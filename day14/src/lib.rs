extern crate day10;

use std::io::Write;

use day10::*;

pub fn count_used(key: &str) -> usize {
    let mut sum: usize = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(key.len() + 4);
    for n in 0..128 {
        write!(&mut buf, "{}-{}", key, n).unwrap();
        let hash = KnotHash::hash(&buf).hash;
        buf.clear();
        hash.iter()
            .for_each(|b| sum += b.count_ones() as usize);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8108, count_used("flqrgnkx"));
    }

    #[test]
    fn puzzle_part_1() {
        assert_eq!(8074, count_used("jzgqcdpd"));
    }
}
