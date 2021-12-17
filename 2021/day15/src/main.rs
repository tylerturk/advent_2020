use cached::proc_macro::cached;
use cached::SizedCache;
use std::collections::HashMap;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[cached(
    type = "SizedCache<String, usize>",
    create = "{SizedCache::with_size(500)}",
    convert = r#"{format!("{}{}", y, x) }"#
)]
fn walk_grid(grid: Vec<Vec<usize>>, path: &mut Vec<Coords>, y: usize, x: usize) -> usize {
    // We know that we haven't started walking yet!
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    let cur = Coords { x, y };
    let cur_risk: usize = match cur {
        Coords { x: 0, y: 0 } => 0,
        _ => *grid.get(y).unwrap().get(x).unwrap(),
    };
    let mut path = path.clone();
    path.push(cur.clone());
    if x == width - 1 && y == height - 1 {
        return cur_risk;
    }
    let mut risks: HashMap<&str, usize> = HashMap::new();

    if x < width - 1 {
        let next = Coords { x: x + 1, y };
        if !path.contains(&next) {
            risks.insert(
                "r",
                cur_risk + walk_grid(grid.clone(), &mut path.clone(), y, x + 1),
            );
        }
    }
    if y < height - 1 {
        let next = Coords { x, y: y + 1 };
        if !path.contains(&next) {
            risks.insert(
                "d",
                cur_risk + walk_grid(grid.clone(), &mut path.clone(), y + 1, x),
            );
        }
    }

    *risks.values().min().unwrap()
}

#[cached(
    type = "SizedCache<String, usize>",
    create = "{SizedCache::with_size(500)}",
    convert = r#"{format!("part2{}{}", y, x) }"#
)]
fn walk_grid_part_2(grid: &Vec<Vec<usize>>, path: &mut Vec<Coords>, y: usize, x: usize) -> usize {
    // We know that we haven't started walking yet!
    let mut path = path.clone();
    if path.len() > 5 {
        path = path.as_slice()[path.len() - 5..path.len()].to_vec();
    }
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    let cur = Coords { x, y };
    let cur_risk: usize = match cur {
        Coords { x: 0, y: 0 } => 0,
        _ => {
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
            // println!(
            //     "Actual: {},{}, transposed: {},{}, size: {},{}",
            //     x, y, xind, yind, width, height
            // );
            let mut val = *grid.get(yind).unwrap().get(xind).unwrap() + modifier;
            loop {
                if val > 9 {
                    val -= 9;
                } else {
                    break;
                }
            }
            val
        }
    };
    // let mut path = path.clone();
    path.push(cur.clone());
    if x == 5 * width - 1 && y == 5 * height - 1 {
        return cur_risk;
    }
    let mut risks: HashMap<&str, usize> = HashMap::new();

    if x < 5 * width - 1 {
        let next = Coords { x: x + 1, y };
        if !path.contains(&next) {
            risks.insert(
                "r",
                cur_risk + walk_grid_part_2(grid, &mut path.clone(), y, x + 1),
            );
        }
    }
    if y < 5 * height - 1 {
        let next = Coords { x, y: y + 1 };
        if !path.contains(&next) {
            risks.insert(
                "d",
                cur_risk + walk_grid_part_2(grid, &mut path.clone(), y + 1, x),
            );
        }
    }
    /*if x > 0 {
        let next = Coords { x: x - 1, y };
        if !path.contains(&next) {
            risks.insert(
                "l",
                cur_risk + walk_grid_part_2(grid, &mut path.clone(), y, x - 1),
            );
        }
    }
    if y > 0 {
        let next = Coords { x, y: y - 1 };
        if !path.contains(&next) {
            risks.insert(
                "u",
                cur_risk + walk_grid_part_2(grid, &mut path.clone(), y - 1, x),
            );
        }
    }*/
    if risks.is_empty() {
        return 9999999999;
    }
    *risks.values().min().unwrap()
}

fn solve_part_1(contents: String) -> usize {
    let grid = generate_map(contents);
    walk_grid(grid, &mut Vec::new(), 0, 0)
}

fn solve_part_2(contents: String) -> usize {
    let grid = generate_map(contents);
    walk_grid_part_2(&grid, &mut Vec::new(), 0, 0)
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
