use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> i32 {
    let positions: Vec<i32> = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut frequency: HashMap<i32, usize> = HashMap::new();
    for pos in &positions {
        *frequency.entry(*pos).or_insert(0) += 1;
    }
    let lowest: i32 = *positions.iter().min().unwrap();
    let highest: i32 = *positions.iter().max().unwrap();
    let mut num_moves: HashMap<i32, i32> = HashMap::new();
    for target in lowest..=highest {
        let mut count: i32 = 0;
        for pos in positions.iter() {
            count += (pos - target).abs();
        }
        num_moves.insert(target, count);
    }
    *num_moves
        .iter()
        .min_by_key(|(_, v)| *v)
        .map(|(_, v)| v)
        .unwrap()
}

fn solve_part_2(contents: String) -> i32 {
    let positions: Vec<i32> = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut frequency: HashMap<i32, usize> = HashMap::new();
    for pos in &positions {
        *frequency.entry(*pos).or_insert(0) += 1;
    }
    let lowest: i32 = *positions.iter().min().unwrap();
    let highest: i32 = *positions.iter().max().unwrap();
    let mut num_moves: HashMap<i32, i32> = HashMap::new();
    for target in lowest..=highest {
        let mut count: i32 = 0;
        for pos in positions.iter() {
            count += determine_fuel((pos - target).abs());
        }
        num_moves.insert(target, count);
    }
    *num_moves
        .iter()
        .min_by_key(|(_, v)| *v)
        .map(|(_, v)| v)
        .unwrap()
}

#[cached]
fn determine_fuel(distance: i32) -> i32 {
    let mut distance = distance;
    let mut fuel: i32 = 0;
    let mut increase: i32 = 1;
    while distance > 0 {
        fuel += increase;
        increase += 1;
        distance -= 1;
    }
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 37);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 168);
    }
}
