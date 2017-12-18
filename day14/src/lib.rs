extern crate day10;

use std::fmt;
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

struct Grid {
    grid: [u8; 2048],
}

impl Grid {
    fn new() -> Grid {
        Grid { grid: [0u8; 2048]}
    }

    fn with_key(key: &str) -> Grid {
        let mut grid = [0u8; 2048];
        {
            let mut grid_it = grid.iter_mut();

            let mut buf: Vec<u8> = Vec::with_capacity(key.len() + 4);
            for n in 0..128 {
                write!(&mut buf, "{}-{}", key, n).unwrap();
                let hash = KnotHash::hash(&buf).hash;
                buf.clear();
                for b in hash.iter() {
                    *grid_it.next().unwrap() = *b;
                }
            }
        }
        Grid { grid }
    }

    fn set(&mut self, x: usize, y: usize) {
        let i = y * 16 + (x >> 3);
        let b = self.grid[i];
        self.grid[i] = b | (0x80 >> (x & 0x07))
    }

    fn is_set(&self, x: usize, y: usize) -> bool {
        let i = y * 16 + (x >> 3);
        let b = self.grid[i];
        0 != b & (0x80 >> (x & 0x07))
    }
}

pub fn count_regions(key: &str) -> usize {
    let mut sum: usize = 0;
    let grid = Grid::with_key(key);
    let mut regions = Grid::new();
    for x in 0..128 {
        for y in 0..128 {
            if set_neighbors(x, y, &grid, &mut regions) {
                sum += 1;
            }
        }
    }
    sum
}

fn set_neighbors(x: usize, y: usize, grid: &Grid, regions: &mut Grid) -> bool {
    if !grid.is_set(x, y) {
        return false;
    }

    if !regions.is_set(x, y) {
        regions.set(x, y);

        if x > 0 {
            set_neighbors(x - 1, y, grid, regions);
        }
        if x < 127 {
            set_neighbors(x + 1, y, grid, regions);
        }
        if y > 0 {
            set_neighbors(x, y - 1, grid, regions);
        }
        if y < 127 {
            set_neighbors(x, y + 1, grid, regions);
        }
        true
    } else {
        false
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..128 {
            for x in 0..128 {
                if self.is_set(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(8108, count_used("flqrgnkx"));
    }

    #[test]
    fn puzzle_part_1() {
        assert_eq!(8074, count_used("jzgqcdpd"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1242, count_regions("flqrgnkx"));
    }

    #[test]
    fn puzzle_part_2() {
        assert_eq!(1212, count_regions("jzgqcdpd"));
    }
}
