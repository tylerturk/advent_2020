use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let pieces = line.split_whitespace();
    let scrambled: Vec<String> = pieces.clone().take(10).map(|x| x.to_string()).collect();
    let key: Vec<String> = pieces
        .clone()
        .skip(11)
        .take(4)
        .map(|x| x.to_string())
        .collect();
    (scrambled, key)
}

fn solve_part_1(contents: String) -> i32 {
    let unique_count: [usize; 4] = [2, 4, 3, 7];
    let mut total: i32 = 0;
    for line in contents.lines() {
        let (_, key): (Vec<String>, Vec<String>) = parse_line(line);
        for item in &key {
            if unique_count.contains(&item.len()) {
                total += 1;
            }
        }
    }
    total
}

#[derive(Debug)]
struct Pieces {
    zero: Vec<char>,
    one: Vec<char>,
    two: Vec<char>,
    three: Vec<char>,
    four: Vec<char>,
    five: Vec<char>,
    six: Vec<char>,
    seven: Vec<char>,
    eight: Vec<char>,
    nine: Vec<char>,
}

impl Pieces {
    fn new() -> Self {
        Self {
            zero: Vec::new(),
            one: Vec::new(),
            two: Vec::new(),
            three: Vec::new(),
            four: Vec::new(),
            five: Vec::new(),
            six: Vec::new(),
            seven: Vec::new(),
            eight: Vec::new(),
            nine: Vec::new(),
        }
    }

    fn determine_letter(&self, map: Vec<char>) -> i32 {
        if self.is_letter(&self.zero, &map) {
            return 0;
        } else if self.is_letter(&self.one, &map) {
            return 1;
        } else if self.is_letter(&self.two, &map) {
            return 2;
        } else if self.is_letter(&self.three, &map) {
            return 3;
        } else if self.is_letter(&self.four, &map) {
            return 4;
        } else if self.is_letter(&self.five, &map) {
            return 5;
        } else if self.is_letter(&self.six, &map) {
            return 6;
        } else if self.is_letter(&self.seven, &map) {
            return 7;
        } else if self.is_letter(&self.eight, &map) {
            return 8;
        } else if self.is_letter(&self.nine, &map) {
            return 9;
        }
        -1
    }

    fn is_letter(&self, letter_vec: &Vec<char>, map: &Vec<char>) -> bool {
        if letter_vec.len() != map.len() {
            return false;
        }
        for letter in letter_vec {
            if !map.contains(&letter) {
                return false;
            }
        }
        true
    }
}

/*
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg
*/

fn solve_part_2(contents: String) -> i32 {
    let mut total: i32 = 0;
    let mut char_assocs: HashMap<&str, char> = HashMap::new();
    let mut mappings: Pieces = Pieces::new();
    for line in contents.clone().lines() {
        let (scrambled, key) = parse_line(line);
        let mut seen: Vec<Vec<char>> = Vec::new();
        let mut letter_count: HashMap<char, i32> = HashMap::new();
        for entry in scrambled {
            let mut sorted = entry.chars().collect::<Vec<char>>();
            sorted.sort_by(|a, b| a.cmp(b));
            match entry.len() {
                2 => mappings.one = sorted.clone(),
                3 => mappings.seven = sorted.clone(),
                4 => mappings.four = sorted.clone(),
                7 => mappings.eight = sorted.clone(),
                _ => (),
            };
            if !seen.contains(&sorted) {
                for char in sorted.clone() {
                    *letter_count.entry(char).or_insert(0) += 1;
                }
            }
            seen.push(sorted.clone());
        }
        let letters = vec![&'a', &'b', &'c', &'d', &'e', &'f', &'g'];
        for sc in mappings.seven.iter() {
            if !mappings.one.contains(&sc) {
                char_assocs.insert("a", *sc);
            }
        }
        for (k, v) in &letter_count {
            match v {
                4 => char_assocs.insert("e", *k),
                6 => char_assocs.insert("b", *k),
                9 => char_assocs.insert("f", *k),
                _ => None,
            };
        }
        for oc in mappings.one.iter() {
            if oc != char_assocs.get("f").unwrap() {
                char_assocs.insert("c", *oc);
            }
        }
        for fc in mappings.four.iter() {
            if fc != char_assocs.get("b").unwrap()
                && fc != char_assocs.get("c").unwrap()
                && fc != char_assocs.get("f").unwrap()
            {
                char_assocs.insert("d", *fc);
            }
        }
        let tmp = char_assocs.clone();
        let found: Vec<&char> = tmp.values().collect();
        for letter in letters {
            if !found.contains(&letter) {
                char_assocs.insert("g", *letter);
            }
        }
        let mut mappings_zero: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("b").unwrap(),
            *char_assocs.get("c").unwrap(),
            *char_assocs.get("e").unwrap(),
            *char_assocs.get("f").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_zero.sort_by(|a, b| a.cmp(b));
        mappings.zero = mappings_zero;
        let mut mappings_two: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("c").unwrap(),
            *char_assocs.get("d").unwrap(),
            *char_assocs.get("e").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_two.sort_by(|a, b| a.cmp(b));
        mappings.two = mappings_two;
        let mut mappings_three: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("c").unwrap(),
            *char_assocs.get("d").unwrap(),
            *char_assocs.get("f").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_three.sort_by(|a, b| a.cmp(b));
        mappings.three = mappings_three;
        let mut mappings_five: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("b").unwrap(),
            *char_assocs.get("d").unwrap(),
            *char_assocs.get("f").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_five.sort_by(|a, b| a.cmp(b));
        mappings.five = mappings_five;
        let mut mappings_six: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("b").unwrap(),
            *char_assocs.get("d").unwrap(),
            *char_assocs.get("e").unwrap(),
            *char_assocs.get("f").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_six.sort_by(|a, b| a.cmp(b));
        mappings.six = mappings_six;
        let mut mappings_nine: Vec<char> = vec![
            *char_assocs.get("a").unwrap(),
            *char_assocs.get("b").unwrap(),
            *char_assocs.get("c").unwrap(),
            *char_assocs.get("d").unwrap(),
            *char_assocs.get("f").unwrap(),
            *char_assocs.get("g").unwrap(),
        ];
        mappings_nine.sort_by(|a, b| a.cmp(b));
        mappings.nine = mappings_nine;

        let mut number: String = "".to_string();
        for entry in key {
            let mut sorted = entry.chars().collect::<Vec<char>>();
            sorted.sort_by(|a, b| a.cmp(b));
            number = format!("{}{}", number, mappings.determine_letter(sorted));
        }
        total += number.parse::<i32>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 26);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 61229);
    }
}
