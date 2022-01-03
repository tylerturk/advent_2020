use std::cmp;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> usize {
    let mut cubes: HashMap<String, bool> = HashMap::new();
    for line in contents.lines() {
        let mut line_bits = line.split_whitespace();
        let op = if line_bits.next().unwrap() == "on" {
            true
        } else {
            false
        };

        let loc = line_bits.next().unwrap();
        let mut loc_bits = loc.split(",");
        let mut x_bits = loc_bits.next().unwrap().split("..");
        let mut x_start: i32 = x_bits
            .next()
            .unwrap()
            .replace("x=", "")
            .parse::<i32>()
            .unwrap();
        if x_start < -50 {
            x_start = -50;
        }
        let mut x_end: i32 = x_bits.next().unwrap().parse::<i32>().unwrap();
        if x_end > 50 {
            x_end = 50;
        }
        let mut y_bits = loc_bits.next().unwrap().split("..");
        let mut y_start: i32 = y_bits
            .next()
            .unwrap()
            .replace("y=", "")
            .parse::<i32>()
            .unwrap();
        if y_start < -50 {
            y_start = -50;
        }
        let mut y_end: i32 = y_bits.next().unwrap().parse::<i32>().unwrap();
        if y_end > 50 {
            y_end = 50;
        }
        let mut z_bits = loc_bits.next().unwrap().split("..");
        let mut z_start: i32 = z_bits
            .next()
            .unwrap()
            .replace("z=", "")
            .parse::<i32>()
            .unwrap();
        if z_start < -50 {
            z_start = -50;
        }
        let mut z_end: i32 = z_bits.next().unwrap().parse::<i32>().unwrap();
        if z_end > 50 {
            z_end = 50;
        }
        if x_start > 50 || x_end < -50 || y_start > 50 || y_end < -50 || z_start > 50 || z_end < -50
        {
            continue;
        }
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                for z in z_start..=z_end {
                    cubes.insert(format!("{},{},{}", x, y, z), op);
                }
            }
        }
    }
    cubes.values().filter(|c| **c).count()
    // let mut cubes: Vec<Cube> = Vec::new();
    // let part_1 = Cube::part_1();
    // for (ind, line) in contents.lines().enumerate() {
    //     let mut cube = part_1.intersection(&Cube::new(line));
    //     if cube.is_none() {
    //         continue;
    //     }
    //     let cube = cube.unwrap();
    //     cube.print();
    //     cubes.push(cube);
    // }
    // let mut other_cubes = cubes.clone();
    // let mut unique: usize = 0;
    // for (ind, cube) in cubes.iter_mut().rev().enumerate() {
    //     if cube.on {
    //         unique += cube.determine_unique_volume(&mut other_cubes);
    //     }
    // }
    // unique
    // cubes.iter().filter(|c| c.on).map(|c| c.determine_unique_volume()).sum()
}

#[derive(Clone, PartialEq)]
struct Cube {
    on: bool,
    x: std::ops::Range<i32>,
    y: std::ops::Range<i32>,
    z: std::ops::Range<i32>,
    size: usize,
}

impl Cube {
    fn new(line: &str) -> Self {
        let mut line_bits = line.split_whitespace();
        let on = if line_bits.next().unwrap() == "on" {
            true
        } else {
            false
        };

        let loc = line_bits.next().unwrap();
        let mut loc_bits = loc.split(",");

        let mut x_bits = loc_bits.next().unwrap().split("..");
        let x_start: i32 = x_bits
            .next()
            .unwrap()
            .replace("x=", "")
            .parse::<i32>()
            .unwrap();
        let x_end: i32 = x_bits.next().unwrap().parse::<i32>().unwrap();
        let x = x_start..x_end + 1;

        let mut y_bits = loc_bits.next().unwrap().split("..");
        let y_start: i32 = y_bits
            .next()
            .unwrap()
            .replace("y=", "")
            .parse::<i32>()
            .unwrap();
        let y_end: i32 = y_bits.next().unwrap().parse::<i32>().unwrap();
        let y = y_start..y_end + 1;

        let mut z_bits = loc_bits.next().unwrap().split("..");
        let z_start: i32 = z_bits
            .next()
            .unwrap()
            .replace("z=", "")
            .parse::<i32>()
            .unwrap();
        let z_end: i32 = z_bits.next().unwrap().parse::<i32>().unwrap();
        let z = z_start..z_end + 1;
        let size: usize = x.len() * y.len() * z.len();
        Self { on, x, y, z, size }
    }

    fn part_1() -> Self {
        Self {
            on: false,
            x: -50..51,
            y: -50..51,
            z: -50..51,
            size: 100 * 100 * 100,
        }
    }

    fn print(&self) {
        println!(
            "Cube: x{}..{}, y{}..{}, z{}..{}",
            self.x.start, self.x.end, self.y.start, self.y.end, self.z.start, self.z.end
        )
    }

    fn intersection(&self, other: &Cube) -> Option<Self> {
        let x_start = cmp::max(self.x.start, other.x.start);
        let y_start = cmp::max(self.y.start, other.y.start);
        let z_start = cmp::max(self.z.start, other.z.start);
        let x_end = cmp::min(self.x.end, other.x.end);
        let y_end = cmp::min(self.y.end, other.y.end);
        let z_end = cmp::min(self.z.end, other.z.end);
        if x_start > x_end || y_start > y_end || z_start > z_end {
            return None;
        }
        let x = x_start..x_end + 1;
        let y = y_start..y_end + 1;
        let z = z_start..z_end + 1;
        println!("Intersection between these two (x:{}..{},y:{}..{},z:{}..{})  (x:{}..{},y:{}..{},z:{}..{}) is  (x:{}..{},y:{}..{},z:{}..{})", self.x.start, self.x.end, self.y.start, self.y.end, self.z.start, self.z.end, other.x.start, other.x.end, other.y.start, other.y.end, other.z.start, other.z.end, x.start, x.end, y.start, y.end, z.start, z.end);
        Some(Self {
            on: true,
            size: x.len() * y.len() * z.len(),
            x,
            y,
            z,
        })
    }

    fn determine_unique_volume(&mut self, others: &mut Vec<Cube>) -> usize {
        let mut intersecting: Vec<Cube> = Vec::new();
        for other in others.iter_mut() {
            let intersection = self.intersection(other);
            if intersection.is_none() {
                continue;
            }
            intersecting.push(other.clone());
        }
        let mut unique_size = self.size;
        let intersecting_dupe = intersecting.clone();
        for (ind, other) in intersecting.iter_mut().enumerate() {
            let mut dupe = intersecting_dupe.clone();
            let mut others = dupe
                .iter()
                .skip(ind + 1)
                .map(|c| c.clone())
                .collect::<Vec<Cube>>();
            unique_size -= other.determine_unique_volume(&mut others);
        }
        unique_size
    }
}

fn solve_part_2(contents: String) -> usize {
    let mut cubes: Vec<Cube> = Vec::new();
    for (ind, line) in contents.lines().enumerate() {
        cubes.push(Cube::new(line));
    }
    let other_cubes = cubes.clone();
    for (ind, cube) in cubes.iter_mut().enumerate() {
        let others = other_cubes.iter().skip(ind + 1);
        for other in others {
            // cube.remove_volume(other);
        }
    }
    cubes.iter().filter(|c| c.on).map(|c| c.size).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 590784);
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_part_2(aoc::read_file("sample2.txt")),
            2758514936282235
        );
    }
}
