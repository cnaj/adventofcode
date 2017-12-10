pub fn compute(n: u32) -> u32 {
    let square_nr = find_square_nr(n);

    if square_nr == 0 {
        return 0;
    }

    let lower_right_prev = lower_right(square_nr - 1);
    let side_len = square_nr * 2;

    let pos_on_side = (n - lower_right_prev - 1) % side_len + 1;
    let middle_of_side = square_nr;
    let steps_to_mid = diff(pos_on_side, middle_of_side);

    steps_to_mid + square_nr
}

fn find_square_nr(n: u32) -> u32 {
    if n == 0 {
        panic!("n must be greater than 0, was {}", n);
    }
    let mut step: u32 = 0;
    loop {
        let lower_right = lower_right(step);
        if n <= lower_right {
            break;
        }
        step += 1;
    }
    step
}

/// Returns the lower-right number of a square. Squares are counted starting from 0 outwards.
///
/// # Examples
///
/// ```
/// use day03::lower_right;
///
/// assert_eq!(1, lower_right(0));
/// assert_eq!(9, lower_right(1));
/// assert_eq!(25, lower_right(2));
/// ```
pub fn lower_right(square_nr: u32) -> u32 {
    (square_nr * 2 + 1).pow(2)
}

fn diff(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(0, compute(1));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(3, compute(12));
    }

    #[test]
    fn it_works_3() {
        assert_eq!(2, compute(23));
    }

    #[test]
    fn it_works_4() {
        assert_eq!(31, compute(1024));
    }

    #[test]
    fn test_find_square_nr() {
        assert_eq!(0, find_square_nr(1));
        assert_eq!(1, find_square_nr(9));
        assert_eq!(2, find_square_nr(25));

        assert_eq!(1, find_square_nr(8));
        assert_eq!(2, find_square_nr(20));
    }
}
