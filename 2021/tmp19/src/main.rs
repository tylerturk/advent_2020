use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

enum Axis {
    x,
    y,
    z,
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<(i32, i32, i32, i32)>,
    shared: i32,
    // location: Option<(usize, usize, usize)>,
    // number: i32,
}

#[derive(PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Scanner {
    fn check_all_permutations(&mut self, other: Scanner) -> bool {
        false
    }

    // fn rotate(&mut self, axis: Axis) {
    //     for beacon in self.beacons {
    //         match axis {

    //         }
    //     }
    // }
    fn sum_beacons(&self, other: &Scanner) -> i32 {
        let mut distances: HashMap<i32, i32> = HashMap::new();
        let mut coords: HashMap<i32, Vec<(i32, i32, i32, i32)>> = HashMap::new();
        for b1 in self.beacons.iter() {
            for b2 in other.beacons.iter() {
                *distances.entry(b1.3 + b2.3).or_insert(0) += 1;
                // *coords.entry(b1.3 + b2.3).and_modify(|e| e.push(b1)).or_insert(vec![b1]);
                // *coords.entry(b1.3 + b2.3).and_modify(|e| e.push(b2));
            }

        }
        println!("Distances: {:#?}", distances.values().collect::<Vec<&i32>>().dedup());
        // let key = distances.iter().max_by_key(|e| e.1).unwrap();
        // println!("{:?}", coords.get(key).unwrap());
        *distances.values().max().unwrap()
    }
}

fn parse_scanners(contents: String) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let mut beacons: Vec<(i32, i32, i32, i32)> = Vec::new();
    // let mut number: i32 = 0;
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            scanners.push(Scanner {
                // number,
                beacons: beacons.clone(),
                shared: 0,
            });
            beacons.clear();
            continue;
        }
        if line.starts_with("---") {
            // println!("line {}", line);
            // number = line.split_whitespace().skip(2).next().unwrap().parse::<i32>().unwrap();
            continue;
        }
        println!("{}", line);
        let mut coords = line.split(",");
        let x = coords.next().unwrap().parse::<i32>().unwrap();
        let y = coords.next().unwrap().parse::<i32>().unwrap();
        let z = coords.next().unwrap().parse::<i32>().unwrap();
        beacons.push((x, y, z, ((x * x + y * y + z * z) as f32).sqrt() as i32));
    }
    scanners
}

fn solve_part_1(contents: String) -> i32 {
    let scanners = parse_scanners(contents);
    println!("{:?}", scanners);
    println!("Scanners 0 and 1 have {} in common", scanners[0].sum_beacons(&scanners[1]));
    0
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 0);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
