fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

// matrix
// forward increases x
// up decreases y
// down increases y
fn solve_part_1(contents: String) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    for line in contents.lines() {
        let mut pieces = line.split(" ");
        let command = pieces.next().unwrap();
        let modifier = pieces.next().unwrap().parse::<i32>().unwrap();
        pos = match command {
            "forward" => (pos.0 + modifier, pos.1),
            "up" => (pos.0, pos.1 - modifier),
            _ => (pos.0, pos.1 + modifier),
        };
    }
    pos.0 * pos.1
}

fn solve_part_2(contents: String) -> i32 {
    let mut pos: (i32, i32, i32) = (0, 0, 0);
    for line in contents.lines() {
        let mut pieces = line.split(" ");
        let command = pieces.next().unwrap();
        let modifier = pieces.next().unwrap().parse::<i32>().unwrap();
        pos = match command {
            "forward" => (pos.0 + modifier, pos.1 + modifier * pos.2, pos.2),
            "up" => (pos.0, pos.1, pos.2 - modifier),
            _ => (pos.0, pos.1, pos.2 + modifier),
        };
    }
    pos.0 * pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 150);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 900);
    }
}
