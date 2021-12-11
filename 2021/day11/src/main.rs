use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

struct Map {
    map: Vec<Vec<Location>>,
    width: usize,
    height: usize,
    flashes: usize,
}

impl Map {
    fn new(contents: String) -> Self {
        let mut height: usize = 0;
        let mut map: Vec<Vec<Location>> = Vec::new();
        for line in contents.lines() {
            let mut row: Vec<Location> = Vec::new();
            let vals: Vec<u32> = line
                .trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();
            for val in vals {
                row.push(Location {
                    val,
                    flashed: false,
                });
            }
            map.push(row);
            height += 1;
        }
        let width = map[0].len();
        Self {
            map,
            width,
            height,
            flashes: 0,
        }
    }

    fn all_flash(&self) -> bool {
        for ri in 0..self.width {
            for ci in 0..self.height {
                if !self.map[ci][ri].flashed {
                    return false;
                }
            }
        }
        true
    }

    fn bump_adjacents(&mut self, ri: i32, ci: i32) {
        let rows = ri - 1..=ri + 1;
        let cols = ci - 1..=ci + 1;
        for row in rows {
            for col in cols.clone() {
                if row == ri && col == ci
                    || row < 0
                    || col < 0
                    || col as usize > self.height - 1
                    || row as usize > self.width - 1
                {
                    continue;
                }
                self.map[col as usize][row as usize].val += 1;
                if self.map[col as usize][row as usize].val > 9
                    && !self.map[col as usize][row as usize].flashed
                {
                    self.map[col as usize][row as usize].flash();
                    self.bump_adjacents(row, col);
                }
            }
        }
    }

    fn flash(&mut self) {
        for ri in 0..self.width {
            for ci in 0..self.height {
                if self.map[ci][ri].val > 9 {
                    self.map[ci][ri].flashed = true;
                    self.bump_adjacents(ri as i32, ci as i32)
                }
            }
        }
    }

    fn increment(&mut self) {
        for ri in 0..self.width {
            for ci in 0..self.height {
                self.map[ci][ri].val += 1;
            }
        }
    }

    fn print(&self) {
        for row in &self.map {
            for col in row {
                print!("{}", col.val);
            }
            println!();
        }
    }

    fn reset_flashes(&mut self) {
        for r in 0..self.width {
            for c in 0..self.height {
                if self.map[c][r].flashed || self.map[c][r].val > 9 {
                    self.flashes += 1;
                    &self.map[c][r].reset_flash();
                }
            }
        }
    }
}

struct Location {
    val: u32,
    flashed: bool,
}

impl Location {
    fn flash(&mut self) {
        self.val = 0;
        self.flashed = true;
    }

    fn reset_flash(&mut self) {
        self.flashed = false;
        self.val = 0;
    }
}

fn solve_part_1(contents: String) -> u32 {
    let mut map = Map::new(contents);
    for _ in 0..100 {
        map.increment();
        map.flash();
        map.reset_flashes();
    }
    map.print();
    map.flashes as u32
}

fn solve_part_2(contents: String) -> usize {
    let mut map = Map::new(contents);
    let mut count: usize = 1;
    loop {
        map.increment();
        map.flash();
        if map.all_flash() {
            return count;
        }
        map.reset_flashes();
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 1656);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 195);
    }
}
