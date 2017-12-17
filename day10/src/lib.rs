pub fn knot_hash(list: &mut [u8], lengths: &str) -> u16 {
    if list.len() < 2 {
        panic!("list must have at least 2 elements");
    }

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
            let mut r: usize = pos + len - 1;
            if r >= list.len() {
                r = r % list.len();
            }

            for _ in 0usize..len / 2 {
                let tmp: u8 = list[l];
                list[l] = list[r];
                list[r] = tmp;

                l += 1;
                if l == list.len() {
                    l = 0;
                }
                if r == 0 {
                    r = list.len() - 1;
                } else {
                    r -= 1;
                }
            }
        }

        pos += len + skip;
        if pos >= list.len() {
            pos = pos % list.len();
        }
        skip += 1;
    }

    list[0] as u16 * list[1] as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = [0u8; 5];
        assert_eq!(12, knot_hash(&mut list, "3,4,1,5"));
    }

    #[test]
    fn puzzle() {
        let mut list = [0u8; 256];
        assert_eq!(15990, knot_hash(&mut list, "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88"));
    }
}
