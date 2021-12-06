use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input(), 80));
    println!("Part 2: {}", solve_part_1(aoc::input(), 256));
}

fn solve_part_1(contents: String, days: i64) -> i64 {
    // The goal is to find how many fish each will make in a period of days
    let fish: Vec<i64> = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    println!("{:?}", fish);
    let mut uniques = fish.clone();
    uniques.dedup();
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for age in uniques {
        let count = count_babies(days - age - 1);
        counts.insert(age, count);
    }
    println!("{:?}", counts);
    fish.iter().map(|x| 1 + counts.get(x).unwrap()).sum()
}

#[cached]
fn count_babies(remaining_days: i64) -> i64 {
    if remaining_days < 0 {
        return 0;
    } else {
        1 + count_babies(remaining_days - 9) + count_babies(remaining_days - 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample(), 80), 5934);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_1(aoc::sample(), 256), 26984457539);
    }
}
