use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_iteration(polymer: &HashMap<String, usize>, instructions: &HashMap<String, String>) -> HashMap<String, usize> {
    let mut mutation: HashMap<String, usize> = HashMap::new();
    for pair in polymer.keys() {
        let letters: Vec<char> = pair.chars().collect();
        let insert = instructions.get(pair).unwrap();
        *mutation.entry(format!("{}{}", letters.first().unwrap(), insert)).or_insert(0) += *polymer.get(pair).unwrap();
        *mutation.entry(format!("{}{}", insert, letters.last().unwrap())).or_insert(0) += *polymer.get(pair).unwrap();
    }
    mutation
}

fn solve_part_1(contents: String) -> usize {
    let mut first_char: char = 'a';
    let mut polymer: HashMap<String, usize> = HashMap::new();
    let mut instructions: HashMap<String, String> = HashMap::new();
    let mut gotten_polymer = false;
    for line in contents.lines() {
        if line.is_empty() {
            gotten_polymer = true;
            continue;
        }
        if !gotten_polymer {
            let line_len = line.len();
            let chars: Vec<char> = line.chars().collect();
            for (ind, c) in chars.iter().enumerate() {
                if ind == 0 {
                    first_char = *c;
                }
                if ind == line_len - 1 {
                    break;
                }
                *polymer.entry(format!("{}{}", c, chars[ind + 1])).or_insert(0) += 1;
            }
        } else {
            let bits: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
            instructions.insert(bits.first().unwrap().to_string(), bits.last().unwrap().to_string());
        }
    }
    for _ in 0..10 {
        polymer = solve_iteration(&polymer, &instructions);
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    *counts.entry(first_char).or_insert(0) += 1;
    for key in polymer.keys() {
        let c: char = *key.chars().collect::<Vec<char>>().last().unwrap();
        *counts.entry(c).or_insert(0) += polymer.get(key).unwrap();
    }
    let max_val = *counts.values().max().unwrap();
    let min_val = *counts.values().min().unwrap();
    max_val - min_val
}

fn solve_part_2(contents: String) -> usize {
    let mut first_char: char = 'a';
    let mut polymer: HashMap<String, usize> = HashMap::new();
    let mut instructions: HashMap<String, String> = HashMap::new();
    let mut gotten_polymer = false;
    for line in contents.lines() {
        if line.is_empty() {
            gotten_polymer = true;
            continue;
        }
        if !gotten_polymer {
            // polymer = line.clone();
            let line_len = line.len();
            let chars: Vec<char> = line.chars().collect();
            for (ind, c) in chars.iter().enumerate() {
                if ind == 0 {
                    first_char = *c;
                }
                if ind == line_len - 1 {
                    break;
                }
                *polymer.entry(format!("{}{}", c, chars[ind + 1])).or_insert(0) += 1;
            }
        } else {
            let bits: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
            instructions.insert(bits.first().unwrap().to_string(), bits.last().unwrap().to_string());
        }
    }
    for _ in 0..40 {
        polymer = solve_iteration(&polymer, &instructions);
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    *counts.entry(first_char).or_insert(0) += 1;
    for key in polymer.keys() {
        let c: char = *key.chars().collect::<Vec<char>>().last().unwrap();
        *counts.entry(c).or_insert(0) += polymer.get(key).unwrap();
    }
    let max_val = *counts.values().max().unwrap();
    let min_val = *counts.values().min().unwrap();
    max_val - min_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 1588);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
