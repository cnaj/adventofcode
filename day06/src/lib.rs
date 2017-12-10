use std::collections::hash_set::HashSet;

pub fn reallocate(banks: &mut [u32]) -> usize {
    let mut configs = HashSet::new();

    let mut count: usize = 0;
    loop {
        if !configs.insert(banks.to_vec()) {
            return count;
        }
        count += 1;

        let mut max: u32 = 0;
        let mut pos: usize = 0;
        for (i, n) in banks.iter().enumerate() {
            if *n > max {
                max = *n;
                pos = i;
            }
        }

        banks[pos] = 0;
        for i in (0..banks.len()).cycle().skip(pos + 1).take(max as usize) {
            banks[i] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut banks = [0u32, 2, 7, 0];
        assert_eq!(5, reallocate(&mut banks));
    }
}
