fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn load_map(contents: String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

fn home_column(color: &char) -> usize {
    match color {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => 0,
    }
}

fn move_cost(color: &char) -> i64 {
    match color {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

fn safe_to_go_home_pos(grid: &Vec<Vec<char>>, color: &char) -> Option<(usize, usize)> {
    let x = home_column(&color);
    let mut first_y_pos = 0;
    for y in 2..grid.len() {
        if grid[y][x] != '#' || &grid[y][x] != color {
            return None;
        }
        if first_y_pos == 0 && &grid[y][x] == color {
            first_y_pos = y - 1;
        }
    }
    Some((first_y_pos, x))
}

fn has_valid_path(grid: &Vec<Vec<char>>, mut loc: (usize, usize), target: (usize, usize)) -> bool {
    let color = &grid[loc.0][loc.1];
    let target = safe_to_go_home_pos(&grid, color);
    if target.is_none() {
        return false;
    }
    let target = target.unwrap();
    let x = home_column(color);
    if target.0 == 1 {
        while loc.0 != target.0 {
            loc.0 += 1;
            if grid[loc.0][loc.1] != '.' {
                return false;
            }
        }
        while loc.1 != target.1 {
            if loc.1 > target.1 {
                loc.1 -= 1;
            } else {
                loc.1 += 1;
            }
            if grid[loc.0][loc.1] != '.' {
                return false;
            }
        }
    } else {
        while loc.1 != x {
            if loc.1 > x {
                loc.1 -= 1;
            }
            if grid[loc.0][loc.1] != '.' {
                return false;
            }
        }
    }
    true
}

fn cost(grid: &Vec<Vec<char>>, loc: (usize, usize), target: (usize, usize)) {
    let cost = ((loc.0 - target.0) as i64).abs()
        + ((loc.1 - target.1) as i64).abs() * move_cost(&grid[loc.0][loc.1]);
}

fn determine_eligible_move_candidates(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut eligible: Vec<(usize, usize)> = Vec::new();
    let amphipods = vec!['A', 'B', 'C', 'D'];
    for (ind, _) in grid[1].iter().enumerate() {
        let color = &grid[1][ind];
        if amphipods.contains(color) {
            let target = safe_to_go_home_pos(&grid, color);
            if target.is_none() {
                continue;
            }
            let target = target.unwrap();
            let valid = has_valid_path(&grid, (1, ind), target);
            if valid {
                eligible.push((1, ind));
            }
        }
    }
    for x in vec![3, 5, 7, 9] {
        for y in 2..grid.len() {
            if amphipods.contains(&grid[y][x]) {
                eligible.push((y, x));
                break;
            }
        }
    }
    eligible
}

fn is_solved(grid: &Vec<Vec<char>>) -> bool {
    let hallway_clear = grid[1].iter().filter(|c| *c != &'#' && *c != &'.').count() == 0;
    let colors = vec!['A', 'B', 'C', 'D'];
    for color in colors {
        let x = home_column(&color);
        for y in 2..grid.len() - 1 {
            if grid[y][x] != color {
                return false;
            }
        }
    }
    hallway_clear
}

fn determine_possible_moves(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> Vec<(usize, usize)> {
    let hallway_locations = vec![(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];
    let mut possible_locs = Vec::new();
    if hallway_locations.contains(&loc) {
        let target = safe_to_go_home_pos(&grid, &grid[loc.0][loc.1]);
        if target.is_none() {
            return Vec::new();
        }
        let target = target.unwrap();
        let valid = has_valid_path(grid, loc, target);
        if !valid {
            return Vec::new();
        }
        let loc_and_cost = (target.0, target.1);
        possible_locs.push(loc_and_cost);
    } else {
        for hallway_location in hallway_locations {
            let valid = has_valid_path(grid, loc, hallway_location);
            if valid {
                possible_locs.push((hallway_location.0, hallway_location.1));
            }
        }
    }
    possible_locs
}

fn solve_part_1(contents: String) -> i32 {
    todo!();
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

// fn solve(
//     grid: &mut Vec<Vec<char>>,
//     start: Option<(usize, usize)>,
//     target: Option<(usize, usize)>,
// ) -> i64 {
//     let mut cost = 0;
//     if start.is_some() && target.is_some() {
//         let (_, move_cost) = has_valid_path(grid, start.unwrap(), target.unwrap());
//         // Necessary to avoid second let statement
//         cost += move_cost;
//     }
//     if is_solved() {
//         return Some();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eligible() {
        let grid = load_map(aoc::sample());
        let eligible = determine_eligible_move_candidates(&grid);
        let mut expected = vec![(2, 3), (2, 5), (2, 7), (2, 9)];
        expected.sort();
        assert_eq!(eligible, expected);

        let grid = load_map(aoc::read_file("extra_inputs/sample_moved.txt"));
        let eligible = determine_eligible_move_candidates(&grid);
        let mut expected = vec![(2, 3), (2, 5), (2, 7), (3, 9)];
        expected.sort();
        assert_eq!(eligible, expected);

        let grid = load_map(aoc::read_file("extra_inputs/sample_a_cannot_move.txt"));
        let eligible = determine_eligible_move_candidates(&grid);
        let mut expected = vec![(2, 5), (2, 7), (3, 9)];
        expected.sort();
        assert_eq!(eligible, expected);
    }

    #[test]
    fn no_possible_moves() {
        let grid = load_map(aoc::read_file("extra_inputs/no_moves.txt"));
        let candidates = determine_eligible_move_candidates(&grid);

        assert_eq!(candidates.is_empty(), false);
    }

    #[test]
    fn solved() {
        let grid = load_map(aoc::read_file("extra_inputs/solved.txt"));
        assert_eq!(is_solved(&grid), true);

        let grid = load_map(aoc::read_file("extra_inputs/almost_solved.txt"));
        assert_eq!(is_solved(&grid), false);
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 0);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
