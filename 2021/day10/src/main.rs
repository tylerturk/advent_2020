use std::collections::HashMap;

#[macro_use]
extern crate maplit;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> i32 {
    let mut points: i32 = 0;
    let point_values: HashMap<char, i32> = hashmap![
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    ];
    let mut open: HashMap<char, char> = hashmap![
        '{' => '}',
        '<' => '>',
        '(' => ')',
        '[' => ']',
    ];
    for line in contents.lines() {
        let mut stack: Vec<char> = Vec::new();
        for letter in line.chars() {
            if open.contains_key(&letter) {
                stack.push(*open.get(&letter).unwrap());
            } else {
                match stack.pop() {
                    Some(last_letter) => {
                        if letter != last_letter {
                            points += point_values.get(&letter).unwrap();
                        }
                    }
                    _ => continue,
                };
            }
        }
    }
    points
}

fn solve_part_2(contents: String) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    let point_values: HashMap<char, i64> = hashmap![
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
    ];
    let mut open: HashMap<char, char> = hashmap![
        '{' => '}',
        '<' => '>',
        '(' => ')',
        '[' => ']',
    ];
    for line in contents.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut points: i64 = 0;
        let mut valid = true;
        for letter in line.chars() {
            if open.contains_key(&letter) {
                stack.push(*open.get(&letter).unwrap());
            } else {
                match stack.pop() {
                    Some(last_letter) => {
                        if letter != last_letter {
                            valid = false;
                            break;
                        }
                    }
                    _ => {
                        valid = false;
                        break;
                    }
                };
            }
        }
        if valid {
            stack.reverse();
            for val in &stack {
                points = points * 5 + point_values.get(&val).unwrap();
            }
            if points != 0 {
                scores.push(points);
            }
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 26397);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 288957);
    }
}
