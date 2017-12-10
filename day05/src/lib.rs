pub fn execute(list: &mut [i32]) -> usize {
    if list.len() == 0 {
        return 0;
    }

    let mut ip = 0;
    let mut count: usize = 0;
    loop {
        let cur = list.get_mut(ip as usize);
        match cur {
            Some(cur) => {
                ip += *cur;
                *cur += 1;
                count += 1;
            },
            None => return count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = [0, 3, 0, 1, -3];
        assert_eq!(5, execute(&mut list));
    }
}
