use pathfinding::dijkstra;
use std::hash::Hash;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn generate_map(contents: String) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in contents.lines() {
        let numbers: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(numbers);
    }
    grid
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn get_val(&self, cave: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
        let width = cave.get(0).unwrap().len();
        let height = cave.len();
        let mut modifier: usize = 0;
        let mut xind = x;
        if x >= width {
            while xind >= width {
                xind -= width;
            }
            modifier += x / width;
        }
        let mut yind = y;
        if y >= height {
            while yind >= height {
                yind -= height;
            }
            modifier += y / height;
        }
        let mut val = *cave.get(yind).unwrap().get(xind).unwrap() + modifier;
        loop {
            if val > 9 {
                val -= 9;
            } else {
                break;
            }
        }
        val
    }

    fn neighbors(&self, cave: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut neighbors: Vec<(Pos, usize)> = Vec::new();
        let width = cave.get(0).unwrap().len();
        let height = cave.len();
        if x > 0 {
            neighbors.push((Pos(x - 1, y), self.get_val(cave, x - 1, y)));
        }
        if y > 0 {
            neighbors.push((Pos(x, y - 1), self.get_val(cave, x, y - 1)));
        }
        if x < 5 * width - 1 {
            neighbors.push((Pos(x + 1, y), self.get_val(cave, x + 1, y)));
        }
        if y < 5 * height - 1 {
            neighbors.push((Pos(x, y + 1), self.get_val(cave, x, y + 1)));
        }
        neighbors
    }
}

fn solve_part_1(contents: String) -> usize {
    let grid = generate_map(contents);
    let width = grid.get(0).unwrap().len() - 1;
    let height = grid.len() - 1;
    let result = dijkstra(
        &Pos(0, 0),
        |p| p.neighbors(&grid),
        |p| *p == Pos(width, height),
    );
    result.expect("dijkstra failed").1
}

fn solve_part_2(contents: String) -> usize {
    let grid = generate_map(contents);
    let width = 5 * grid.get(0).unwrap().len() - 1;
    let height = 5 * grid.len() - 1;
    let result = dijkstra(
        &Pos(0, 0),
        |p| p.neighbors(&grid),
        |p| *p == Pos(width, height),
    );
    result.expect("Shit blew up").1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 40);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 315);
    }
}
