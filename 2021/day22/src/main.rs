use std::cmp;
use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> usize {
    let mut cubes: Vec<Cube> = Vec::new();
    let part_1 = Cube::part_1();
    for line in contents.lines() {
        let tmp_cube = Cube::new(line);
        let cube = part_1.intersection(&tmp_cube);
        if cube.is_none() {
            continue;
        }
        let mut cube = cube.unwrap();
        cube.on = tmp_cube.on;
        cubes.push(cube);
    }
    let mut unique: usize = 0;
    for (ind, cube) in cubes.iter().enumerate() {
        if cube.on {
            let remaining_cubes = cubes
                .iter()
                .skip(ind + 1)
                .map(|c| c.clone())
                .collect::<Vec<Cube>>();
            unique += cube.determine_unique_volume(&remaining_cubes);
        }
    }
    unique
}

#[derive(Clone, Debug, PartialEq)]
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
        let x = -50..51;
        let y = -50..51;
        let z = -50..51;
        Self {
            on: false,
            size: x.len() * y.len() * z.len(),
            x,
            y,
            z,
        }
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
        let x = x_start..x_end;
        let y = y_start..y_end;
        let z = z_start..z_end;
        Some(Self {
            on: true,
            size: x.len() * y.len() * z.len(),
            x,
            y,
            z,
        })
    }

    fn determine_unique_volume(&self, others: &Vec<Cube>) -> usize {
        let mut intersecting: Vec<Cube> = Vec::new();
        for other in others.iter() {
            let intersection = self.intersection(other);
            if intersection.is_none() {
                continue;
            }
            intersecting.push(intersection.unwrap());
        }

        if intersecting.is_empty() {
            return self.size;
        }
        let mut other_intersection_sizes: usize = 0;
        for (ind, other) in intersecting.iter().enumerate() {
            let others = intersecting
                .iter()
                .skip(ind + 1)
                .map(|c| c.clone())
                .collect::<Vec<Cube>>();
            other_intersection_sizes += other.determine_unique_volume(&others);
        }
        self.size - other_intersection_sizes
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cube: x{}..{}, y{}..{}, z{}..{}",
            self.x.start, self.x.end, self.y.start, self.y.end, self.z.start, self.z.end
        )
    }
}

fn solve_part_2(contents: String) -> usize {
    let mut cubes: Vec<Cube> = Vec::new();
    for line in contents.lines() {
        cubes.push(Cube::new(line));
    }
    let mut unique: usize = 0;
    for (ind, cube) in cubes.iter().enumerate() {
        if cube.on {
            let remaining_cubes = cubes
                .iter()
                .skip(ind + 1)
                .map(|c| c.clone())
                .collect::<Vec<Cube>>();
            unique += cube.determine_unique_volume(&remaining_cubes);
        }
    }
    unique
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

    #[test]
    fn test_volume() {
        let line = "on x=1..1,y=1..1,z=1..1";
        let c1 = Cube::new(line);
        assert_eq!(c1.size, 1);

        let line = "on x=-1..1,y=-1..1,z=-1..1";
        let c2 = Cube::new(line);
        assert_eq!(c2.size, 27);

        assert_eq!(c2.determine_unique_volume(&vec![c1.clone()]), 26);

        let c1_c2 = c2.intersection(&c1);
        assert_eq!(c1_c2.is_some(), true);

        let line = "on x=-2..3,y=-2..3,z=-2..3";
        let c3 = Cube::new(line);
        assert_eq!(c3.size, 216);

        let c2_c3 = c2.intersection(&c3);
        assert_eq!(c2_c3.unwrap().size, 27);
    }

    #[test]
    fn test_simple() {
        assert_eq!(solve_part_1(aoc::read_file("simple.txt")), 26);
    }
}
