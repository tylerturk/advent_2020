use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(coords: &str) -> Self {
        let mut coords = coords.split(",");
        Self {
            x: coords.next().unwrap().parse::<i32>().unwrap(),
            y: coords.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

fn parse_line(line: &str) -> Line {
    let mut line_bits = line.split_whitespace();
    let first_point = Point::new(line_bits.next().unwrap());
    let second_point = Point::new(line_bits.last().unwrap());
    if first_point.x < second_point.x || first_point.y < second_point.y {
        Line { start: first_point, end: second_point }
    } else {
        Line { start: second_point, end: first_point }
    }
}

fn solve_part_1(contents: String) -> i32 {
    let mut locations: HashMap<String, i32> = HashMap::new();
    for line in contents.lines() {
        let cur_line = parse_line(line);
        // println!("{:?}", cur_line);
        if cur_line.start.x == cur_line.end.x {
            // This is a vertical line
            for y in cur_line.start.y..=cur_line.end.y {
                *locations.entry(format!("{}:{}", cur_line.start.x, y)).or_insert(0) += 1;
            }
        } else if cur_line.start.y == cur_line.end.y {
            // This is a horizontal line
            for x in cur_line.start.x..=cur_line.end.x {
                *locations.entry(format!("{}:{}", x, cur_line.start.y)).or_insert(0) += 1;
            }
        }
    }
    // println!("{:?}", locations);
    let mut dupes: i32 = 0;
    for (_, v) in locations.iter() {
        if v > &1 {
            dupes += 1;
        }
    }
    dupes
}

fn solve_part_2(contents: String) -> i32 {
    let mut locations: HashMap<String, i32> = HashMap::new();
    for line in contents.lines() {
        let cur_line = parse_line(line);
        // println!("{:?}", cur_line);
        if cur_line.start.x == cur_line.end.x {
            // This is a horizontal line
            for y in cur_line.start.y..=cur_line.end.y {
                *locations.entry(format!("{}:{}", cur_line.start.x, y)).or_insert(0) += 1;
            }
        } else if cur_line.start.y == cur_line.end.y {
            // This is a vertical line
            for x in cur_line.start.x..=cur_line.end.x {
                *locations.entry(format!("{}:{}", x, cur_line.start.y)).or_insert(0) += 1;
            }
        } else {
            if cur_line.start.x < cur_line.end.x {
                let mut y = cur_line.start.y;
                for x in cur_line.start.x..=cur_line.end.x {
                    *locations.entry(format!("{}:{}", x, y)).or_insert(0) += 1;
                    if cur_line.start.y < cur_line.end.y {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                }
            } else {
                let mut x = cur_line.start.x;
                for y in cur_line.start.y..=cur_line.end.y {
                    *locations.entry(format!("{}:{}", x, y)).or_insert(0) += 1;
                    if cur_line.start.x < cur_line.end.x {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                }
            }
        }
    }
    let mut dupes: i32 = 0;
    for (_, v) in locations.iter() {
        if v > &1 {
            dupes += 1;
        }
    }
    dupes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 5);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 12);
    }
}
