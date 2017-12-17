pub struct KnotHash {
    list: [u8; 256],
}

impl KnotHash {
    pub fn hash(lengths: &str) -> KnotHash {
        let mut list = [0u8; 256];

        let mut cur = 0u8;
        for el in list.iter_mut() {
            *el = cur;
            cur = cur.wrapping_add(1);
        }

        let mut pos = 0usize;
        let mut skip = 0usize;
        for len in lengths.split(',')
            .map(|s| usize::from_str_radix(s, 10).unwrap()) {

            if len > 0 {
                let mut l: usize = pos;
                let mut r: usize = (pos + len - 1) & 0xff;

                for _ in 0usize..len / 2 {
                    let tmp: u8 = list[l];
                    list[l] = list[r];
                    list[r] = tmp;

                    l = (l + 1) & 0xff;
                    r = (r + 0xff) & 0xff;
                }
            }

            pos = (pos + len + skip) & 0xff;
            skip = (skip + 1) & 0xff;
        }

        KnotHash { list }
    }

    pub fn checksum(&self) -> u16 {
        self.list[0] as u16 * self.list[1] as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle() {
        let hash = KnotHash::hash("183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88");
        assert_eq!(15990, hash.checksum());
    }
}
