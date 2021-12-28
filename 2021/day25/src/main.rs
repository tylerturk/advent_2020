fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Debug)]
struct SeaFloor {
    grid: Vec<Vec<char>>,
    height: usize,
    shifts: i32,
    width: usize,
}

#[derive(Clone, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

impl SeaFloor {
    fn new(contents: &String) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in contents.lines() {
            grid.push(line.chars().collect())
        }
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            shifts: 0,
            width,
        }
    }

    fn print(&self) {
        println!("After {} shifts: ", self.shifts);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y][x]);
            }
            println!("");
        }
        println!("");
    }

    fn eastbound(&mut self) -> bool {
        let mut prev_pos: Vec<Coords> = Vec::new();
        let mut new_pos: Vec<Coords> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coords { x, y };
                if self.grid[y][x] != '>' || new_pos.contains(&coords) {
                    continue;
                }
                let new_x = if x + 1 == self.width {
                    0
                } else {
                    x + 1
                };
                if self.grid[y][new_x] == '.' {
                    self.grid[y][new_x] = '>';
                    prev_pos.push(coords);
                    new_pos.push(Coords { x: new_x, y });
                }
            }
        }
        for coord in prev_pos {
            self.grid[coord.y][coord.x] = '.';
        }
        new_pos.len() > 0
    }

    fn southbound(&mut self) -> bool {
        let mut prev_pos: Vec<Coords> = Vec::new();
        let mut new_pos: Vec<Coords> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coords { x, y };
                if self.grid[y][x] != 'v' || new_pos.contains(&coords) {
                    continue;
                }
                let new_y = if y + 1 == self.height {
                    0
                } else {
                    y + 1
                };
                if self.grid[new_y][x] == '.' {
                    self.grid[new_y][x] = 'v';
                    prev_pos.push(coords);
                    new_pos.push(Coords { x, y: new_y });
                }
            }
        }
        for coord in prev_pos {
            self.grid[coord.y][coord.x] = '.';
        }
        new_pos.len() > 0
    }

    fn shift(&mut self) -> bool {
        let eb = self.eastbound();
        let sb = self.southbound();
        eb || sb
    }

    fn keep_shifting(&mut self) {
        loop {
            if self.shift() {
                self.shifts += 1;
                // self.print();
            } else {
                break;
            }
        }
    }
}

fn solve_part_1(contents: String) -> i32 {
    let mut sf = SeaFloor::new(&contents);
    sf.keep_shifting();
    sf.shifts + 1
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 58);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
