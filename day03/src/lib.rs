pub fn compute(n: u32) -> i32 {
    let (x, y) = find_grid_pos(n);
    x.abs() + y.abs()
}

/// Returns the element number of the given position, starting with 1 at (0, 0).
///
/// # Examples
///
/// ```
/// use day03::find_n;
///
/// assert_eq!( 1, find_n( 0,  0));
/// assert_eq!( 8, find_n( 0, -1));
/// assert_eq!( 9, find_n( 1, -1));
/// assert_eq!(10, find_n( 2, -1));
/// assert_eq!(12, find_n( 2,  1));
/// assert_eq!(25, find_n( 2, -2));
/// assert_eq!(20, find_n(-2, -1));
/// assert_eq!(23, find_n( 0, -2));
/// ```
pub fn find_n(x: i32, y: i32) -> u32 {
    if (x, y) == (0, 0) {
        return 1;
    }

    let ring;
    let pos_on_ring;
    if x >= y && x > -y {
        // side 0
        ring = x as u32;
        pos_on_ring = (x + y - 1) as u32;
    } else if -x > -y && x >= -y {
        // side 1
        ring = y as u32;
        pos_on_ring = (y - x - 1) as u32 + ring * 2;
    } else if -x > y && -x >= -y {
        // side 2
        ring = -x as u32;
        pos_on_ring = (-x - y - 1) as u32 + ring * 4;
    } else {
        // side 3
        ring = -y as u32;
        pos_on_ring = (x + - y - 1) as u32 + ring * 6;
    }

    // (2r + 1)² - (2r - 1)² = 8r
    let total_on_ring = 8 * ring;

    lower_right(ring) + 1 - total_on_ring + pos_on_ring
}

/// Returns the position of the number on the grid, with the access port at position (0, 0).
///
/// # Examples
///
/// ```
/// use day03::find_grid_pos;
///
/// assert_eq!(( 0,  0), find_grid_pos( 1));
/// assert_eq!(( 0, -1), find_grid_pos( 8));
/// assert_eq!(( 1, -1), find_grid_pos( 9));
/// assert_eq!(( 2, -1), find_grid_pos(10));
/// assert_eq!(( 2,  1), find_grid_pos(12));
/// assert_eq!(( 2, -2), find_grid_pos(25));
/// assert_eq!((-2, -1), find_grid_pos(20));
/// assert_eq!(( 0, -2), find_grid_pos(23));
/// ```
pub fn find_grid_pos(n: u32) -> (i32, i32) {
    let ring = find_ring(n);
    if ring == 0 {
        return (0, 0);
    }

    let lower_right_prev = lower_right(ring - 1);
    let side_len = ring * 2;

    let n_on_ring = n - lower_right_prev - 1;

    let pos_on_side = (n_on_ring % side_len) + 1;
    let middle_of_side = ring as i32;
    let side_delta = pos_on_side as i32 - middle_of_side;

    match n_on_ring / side_len {
        0 => (middle_of_side, side_delta),
        1 => (-side_delta, middle_of_side),
        2 => (-middle_of_side, -side_delta),
        3 => (side_delta, -middle_of_side),
        _ => panic!("unexpected"),
    }
}

fn find_ring(n: u32) -> u32 {
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
        assert_eq!(0, find_ring(1));
        assert_eq!(1, find_ring(9));
        assert_eq!(2, find_ring(25));

        assert_eq!(1, find_ring(8));
        assert_eq!(2, find_ring(20));
    }
}
