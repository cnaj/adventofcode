use std::fmt;

pub fn hash_str(string: &str) -> String {
    let hash = KnotHash::hash_str(string);
    format!("{}", hash)
}

pub fn hash(input: &[u8]) -> String {
    let hash = KnotHash::hash(input);
    format!("{}", hash)
}

pub struct KnotHash {
    pub hash: [u8; 16],
}

impl KnotHash {
    pub fn hash_str(string: &str) -> KnotHash {
        KnotHash::hash(string.as_bytes())
    }

    pub fn hash(input: &[u8]) -> KnotHash {
        let suffix: [u8; 5] = [17, 31, 73, 47, 23];
        let mut list = [0u8; 256];

        let mut cur = 0u8;
        for el in list.iter_mut() {
            *el = cur;
            cur = cur.wrapping_add(1);
        }

        let mut pos = 0usize;
        let mut skip = 0usize;

        for _ in 0..64 {
            let len_it = input.iter()
                .chain(suffix.iter())
                .map(|b| *b as usize);

            for len in len_it {
                if len > 0 {
                    let mut l: usize = pos;
                    let mut r: usize = (pos + len - 1) & 0xff;

                    for _ in 0usize..len / 2 {
                        list.swap(l, r);
                        l = (l + 1) & 0xff;
                        r = (r + 0xff) & 0xff;
                    }
                }

                pos = (pos + len + skip) & 0xff;
                skip = (skip + 1) & 0xff;
            }
        }

        // condense hash

        let mut hash = [0u8; 16];
        {
            let mut hash_it = hash.iter_mut();
            for chunk in list.chunks(16) {
                let a = chunk[0];
                let x = chunk.iter()
                    .skip(1)
                    .fold(a, |a, b| a ^ b);
                *hash_it.next().unwrap() = x;
            }
        }

        KnotHash { hash }
    }
}

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.hash.iter() {
            write!(f, "{:02x}", x)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", &hash_str(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", &hash_str("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", &hash_str("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", &hash_str("1,2,4"));
    }
}
