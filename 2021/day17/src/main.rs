use regex::Regex;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn new(contents: String) -> Self {
        let re = Regex::new("[-0-9]+..[-0-9]+").unwrap();
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        for mat in re.find_iter(&contents) {
            let snippet = &contents[mat.start()..mat.end()];
            let bits: Vec<&str> = snippet.split("..").collect();
            if x_min == x_max {
                x_min = bits.first().unwrap().parse::<i32>().unwrap();
                x_max = bits.last().unwrap().parse::<i32>().unwrap();
            } else {
                y_min = bits.first().unwrap().parse::<i32>().unwrap();
                y_max = bits.last().unwrap().parse::<i32>().unwrap();
            }
        }
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn in_x(&self, x: i32) -> bool {
        if self.x_min <= x && x <= self.x_max {
            return true;
        }
        false
    }

    fn contains(&self, coords: &Coords) -> bool {
        if self.x_min <= coords.x && coords.x <= self.x_max && self.y_min <= coords.y && coords.y <= self.y_max {
            return true;
        }
        false
    }

    fn overshot(&self, coords: &Coords) -> bool {
        if coords.x > self.x_max || coords.y < self.y_min {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn combine(&mut self, delta: &Coords) {
        self.x += delta.x;
        self.y += delta.y;
    }

    fn slow(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
        self.y -= 1;
    }
}

fn test_shot(area: &TargetArea, trajectory: &mut Coords) -> Option<i32> {
    let mut max_height = 0;
    let mut pos = Coords { x: 0, y: 0 };
    loop {
        if trajectory.x == 0 && !area.in_x(pos.x) {
            return None;
        } else if area.overshot(&pos) {
            return None;
        }
        pos.combine(trajectory);
        if area.contains(&pos) {
            break;
        }
        trajectory.slow();
        if &pos.y > &max_height {
            max_height = pos.y;
        }

    }
    Some(max_height)
}

fn solve_part_1(contents: String) -> i32 {
    let area = TargetArea::new(contents);
    let trajectory_max = area.y_min * -1;
    let mut heights: Vec<i32> = Vec::new();
    for x in 0..=area.x_max {
        for y in 0..trajectory_max {
            let mut coords  = Coords {x, y};
            match test_shot(&area, &mut coords) {
                Some(height) => heights.push(height),
                _ => (),
            }
        }
    }
    *heights.iter().max().unwrap()
}

fn solve_part_2(contents: String) -> usize {
    let area = TargetArea::new(contents);
    let trajectory_max = area.y_min * -1;
    let mut heights: Vec<i32> = Vec::new();
    let mut trajectories: Vec<Coords> = Vec::new();
    for x in 0..=area.x_max {
        for y in area.y_min..trajectory_max {
            let mut coords  = Coords {x, y};
            match test_shot(&area, &mut coords) {
                Some(height) => {
                    heights.push(height);
                    trajectories.push(coords);
                }
                _ => (),
            }
        }
    }
    heights.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 45);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 112);
    }
}
