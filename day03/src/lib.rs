pub fn compute(n: u32) -> i32 {
    let (x, y) = find_grid_pos(n);
    x.abs() + y.abs()
}

/// Returns the position of the number on the grid, with the access port at position (0, 0).
///
/// # Examples
///
/// ```
/// use day03::find_grid_pos;
///
/// assert_eq!((0, 0), find_grid_pos(1));
/// assert_eq!((0, -1), find_grid_pos(8));
/// assert_eq!((1, -1), find_grid_pos(9));
/// assert_eq!((2, -1), find_grid_pos(10));
/// assert_eq!((2, 1), find_grid_pos(12));
/// assert_eq!((2, -2), find_grid_pos(25));
/// assert_eq!((-2, -1), find_grid_pos(20));
/// assert_eq!((0, -2), find_grid_pos(23));
/// ```
pub fn find_grid_pos(n: u32) -> (i32, i32) {
    let ring_nr = find_ring_nr(n);
    if ring_nr == 0 {
        return (0, 0);
    }

    let lower_right_prev = lower_right(ring_nr - 1);
    let side_len = ring_nr * 2;

    let n_on_ring = n - lower_right_prev - 1;

    let pos_on_side = (n_on_ring % side_len) + 1;
    let middle_of_side = ring_nr as i32;
    let side_delta = pos_on_side as i32 - middle_of_side;

    match n_on_ring / side_len {
        0 => (middle_of_side, side_delta),
        1 => (-side_delta, middle_of_side),
        2 => (-middle_of_side, -side_delta),
        3 => (side_delta, -middle_of_side),
        _ => panic!("unexpected"),
    }
}

fn find_ring_nr(n: u32) -> u32 {
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

/// Returns the lower-right number of a ring. Rings are counted starting from 0 outwards.
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
pub fn lower_right(ring_nr: u32) -> u32 {
    (ring_nr * 2 + 1).pow(2)
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
    fn test_find_ring_nr() {
        assert_eq!(0, find_ring_nr(1));
        assert_eq!(1, find_ring_nr(9));
        assert_eq!(2, find_ring_nr(25));

        assert_eq!(1, find_ring_nr(8));
        assert_eq!(2, find_ring_nr(20));
    }
}
