use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn traverse_to_end(
    caves: HashMap<String, Vec<String>>,
    current: &String,
    prev_path: &Vec<String>,
) -> i32 {
    let mut path: Vec<String> = prev_path.clone();
    let mut valid_paths = 0;
    path.push(current.clone());
    if current == &"end".to_string() {
        return 1;
    }
    for cave in caves.get(current).unwrap() {
        let contains_cave = path.contains(cave);
        if contains_cave && cave == &cave.to_uppercase() || !contains_cave {
            valid_paths += traverse_to_end(caves.clone(), cave, &path)
        }
    }
    valid_paths
}

fn solve_part_1(contents: String) -> i32 {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in contents.lines() {
        let bits: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        caves
            .entry(bits.first().unwrap().to_string())
            .or_insert(Vec::new())
            .push(bits.last().unwrap().to_string());
        caves
            .entry(bits.last().unwrap().to_string())
            .or_insert(Vec::new())
            .push(bits.first().unwrap().to_string());
    }
    traverse_to_end(caves, &"start".to_string(), &Vec::new())
}

fn traverse_to_end_2(
    caves: HashMap<String, Vec<String>>,
    current: &String,
    prev_path: &Vec<String>,
    small_cave_visit_used: bool,
) -> i32 {
    let mut path: Vec<String> = prev_path.clone();
    let mut valid_paths = 0;
    path.push(current.clone());
    if current == &"end".to_string() {
        return 1;
    }
    for cave in caves.get(current).unwrap() {
        let contains_cave = path.contains(cave);
        let cave_count = path.iter().filter(|c| c == &cave).count();
        if contains_cave && cave == &cave.to_uppercase() || !contains_cave {
            valid_paths += traverse_to_end_2(caves.clone(), cave, &path, small_cave_visit_used)
        } else if contains_cave && cave_count < 2 && cave != &"start" && !small_cave_visit_used {
            valid_paths += traverse_to_end_2(caves.clone(), cave, &path, true)
        }
    }
    valid_paths
}

fn solve_part_2(contents: String) -> i32 {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in contents.lines() {
        let bits: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        caves
            .entry(bits.first().unwrap().to_string())
            .or_insert(Vec::new())
            .push(bits.last().unwrap().to_string());
        caves
            .entry(bits.last().unwrap().to_string())
            .or_insert(Vec::new())
            .push(bits.first().unwrap().to_string());
    }
    traverse_to_end_2(caves, &"start".to_string(), &Vec::new(), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 10);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 36);
    }
}
