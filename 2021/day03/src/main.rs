use itertools::join;
use std::convert::TryInto;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> u64 {
    // let numbers: Vec<u64> = contents.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let numbers: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    let entries = numbers.len();
    let mut count: Vec<u64> = Vec::new();
    // let total = numbers.iter().sum::<u64>().to_string();
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    // println!("total: {}", total);
    let mut use_push = true;
    for number in numbers {
        if !count.is_empty() {
            use_push = false;
        }
        for (ind, letter) in number.iter().enumerate() {
            if use_push {
                count.push(letter.to_string().parse::<u64>().unwrap());
            } else {
                count[ind] += letter.to_string().parse::<u64>().unwrap();
            }
        }
    }

    for letter in count {
        let letter = letter.to_string().parse::<u64>().unwrap();
        let mut g_char = "0";
        let mut e_char = "1";
        if letter > (entries / 2).try_into().unwrap() {
            g_char = "1";
            e_char = "0";
        }
        gamma = format!("{}{}", gamma, g_char);
        epsilon = format!("{}{}", epsilon, e_char);
    }
    println!("gamma: {}, epsilon: {}", &gamma, &epsilon);
    u64::from_str_radix(&gamma, 2).unwrap() * u64::from_str_radix(&epsilon, 2).unwrap()
}

fn solve_part_2(contents: String) -> i32 {
    let mut most: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    most.sort();
    let mut least: Vec<Vec<char>> = most.clone();
    let length = most.get(0).unwrap().len();
    for ind in 0..length {
        if most.len() > 1 {
            most = most_frequent(most, ind);
        }
        if least.len() > 1 {
            least = least_frequent(least, ind);
        }
    }
    let most_as_int: isize = isize::from_str_radix(&join(most.get(0).unwrap(), ""), 2).unwrap();
    let least_as_int: isize = isize::from_str_radix(&join(least.get(0).unwrap(), ""), 2).unwrap();
    most_as_int as i32 * least_as_int as i32
}

struct Counts {
    ones: i32,
    zeroes: i32,
}

fn most_frequent(numbers: Vec<Vec<char>>, ind: usize) -> Vec<Vec<char>> {
    let mut counts = Counts { ones: 0, zeroes: 0 };
    for number in &numbers {
        if number.get(ind).unwrap() == &'1' {
            counts.ones += 1;
        } else {
            counts.zeroes += 1;
        }
    }
    if counts.ones >= counts.zeroes {
        numbers
            .into_iter()
            .filter(|n| n.get(ind).unwrap() == &'1')
            .collect()
    } else {
        numbers
            .into_iter()
            .filter(|n| n.get(ind).unwrap() == &'0')
            .collect()
    }
}

fn least_frequent(numbers: Vec<Vec<char>>, ind: usize) -> Vec<Vec<char>> {
    let mut counts = Counts { ones: 0, zeroes: 0 };
    for number in &numbers {
        if number.get(ind).unwrap() == &'1' {
            counts.ones += 1;
        } else {
            counts.zeroes += 1;
        }
    }
    if counts.ones < counts.zeroes {
        numbers
            .into_iter()
            .filter(|n| n.get(ind).unwrap() == &'1')
            .collect()
    } else {
        numbers
            .into_iter()
            .filter(|n| n.get(ind).unwrap() == &'0')
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 198);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 230);
    }
}
