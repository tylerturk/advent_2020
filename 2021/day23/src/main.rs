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

fn color_column(col: usize) -> char {
    match col {
        3 => 'A',
        5 => 'B',
        7 => 'C',
        9 => 'D',
        _ => '.',
    }
}

fn colors_move_cost(color: &char) -> i64 {
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
        if grid[y][x] != '#' && &grid[y][x] != color && grid[y][x] != '.' {
            return None;
        }
        if first_y_pos == 0 && &grid[y][x] == color {
            first_y_pos = y - 1;
        }
    }
    Some((first_y_pos, x))
}

fn has_valid_path(grid: &Vec<Vec<char>>, mut loc: (usize, usize), target: (usize, usize)) -> bool {
    if target.0 == 1 {
        while loc.0 != target.0 {
            loc.0 -= 1;
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
        while loc.1 != target.1 {
            if loc.1 > target.1 {
                loc = (loc.0, loc.1 - 1);
            } else {
                loc = (loc.0, loc.1 + 1);
            }
            if grid[loc.0][loc.1] != '.' {
                return false;
            }
        }
        while loc.0 != target.0 {
            loc = (loc.0 + 1, loc.1);
            if grid[loc.0][loc.1] != '.' {
                return false;
            }
        }
    }
    true
}

fn determine_move_cost(color: &char, loc: (usize, usize), target: (usize, usize)) -> i64 {
    ((loc.0 as i64 - target.0 as i64).abs() + (loc.1 as i64 - target.1 as i64).abs())
        * colors_move_cost(color)
}

fn determine_eligible_move_candidates(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut eligible: Vec<(usize, usize)> = Vec::new();
    let amphipods = vec!['A', 'B', 'C', 'D'];
    for (ind, _) in grid[1].iter().enumerate() {
        let color = &grid[1][ind];
        if amphipods.contains(color) {
            let target = safe_to_go_home_pos(&grid, color);
            if target.is_none() {
                println!("No safe spot found for {},{}", 1, ind);
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
        let mut valid_column = true;
        for y in (2..grid.len() - 1).rev() {
            let col = &grid[y][x];
            if amphipods.contains(col) && home_column(col) != x {
                valid_column = false;
            }
        }
        if valid_column {
            continue;
        }
        for y in 2..grid.len() {
            if amphipods.contains(&grid[y][x]) && !amphipod_is_solved(&grid, &color_column(x)) {
                eligible.push((y, x));
                break;
            }
        }
    }
    eligible
}

fn amphipod_is_solved(grid: &Vec<Vec<char>>, color: &char) -> bool {
    let x = home_column(&color);
    for y in 2..grid.len() - 1 {
        if grid[y][x] != *color {
            return false;
        }
    }
    true
}

fn is_solved(grid: &Vec<Vec<char>>) -> bool {
    let hallway_clear = grid[1].iter().filter(|c| *c != &'#' && *c != &'.').count() == 0;
    let colors = vec!['A', 'B', 'C', 'D'];
    for color in colors {
        if !amphipod_is_solved(&grid, &color) {
            return false;
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

fn solve_part_1(contents: String) -> i64 {
    let mut grid = load_map(contents);
    let ans = solve(&mut grid, None, None, 0);
    if ans.is_some() {
        return ans.unwrap();
    }
    -1
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for (y, _) in grid.iter().enumerate() {
        for (x, _) in grid[y].iter().enumerate() {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
}

fn solve(
    grid: &Vec<Vec<char>>,
    start: Option<(usize, usize)>,
    target: Option<(usize, usize)>,
    depth: i32,
) -> Option<i64> {
    println!("\n\nIteration {}", depth);
    print_grid(&grid);
    let mut cost = 0;
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for (y, _) in grid.iter().enumerate() {
        new_grid.push(grid[y].iter().map(|c| c.clone()).collect());
    }

    if start.is_some() && target.is_some() {
        let start = start.unwrap();
        let target = target.unwrap();
        cost += determine_move_cost(&grid[start.0][start.1], start, target);
        // Necessary to avoid second let statement
        new_grid[target.0][target.1] = grid[start.0][start.1];
        new_grid[start.0][start.1] = '.';
    }
    if is_solved(&grid) {
        println!("Solved with cost {}", cost);
        return Some(cost);
    }
    let candidates = determine_eligible_move_candidates(&grid);
    if candidates.is_empty() {
        println!("Exiting because candidates have no valid options");
        return None;
    }
    let mut costs: Vec<i64> = Vec::new();
    for candidate in candidates.clone().iter() {
        let moves = determine_possible_moves(&grid, *candidate);
        for (i, m) in moves.clone().iter().enumerate() {
            println!(
                "Depth {}, iteration {}, Calling solve on {:?} -> {:?} ",
                depth, i, candidate, m
            );
            let ans = solve(&new_grid, Some(*candidate), Some(*m), depth + 1);
            if ans.is_some() {
                costs.push(ans.unwrap());
            }
            // match solve(
            //     &new_grid.clone(),
            //     Some(candidate.clone()),
            //     Some(m.clone()),
            //     depth + 1,
            // ) {
            //     Some(val) => costs.push(val),
            //     None => println!("Unable to solve with {:?} going to {:?}", candidate, m,),
            // };
        }
    }
    print_grid(&grid);
    Some(costs.iter().min().unwrap() + cost)
}

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
    fn is_valid_move() {
        let grid = load_map(aoc::read_file("extra_inputs/almost_solved.txt"));
        let start = (1, 10);
        let target = (2, 9);
        assert_eq!(has_valid_path(&grid, start, target), true);

        let grid = load_map(aoc::sample());
        let start = (2, 5);
        let target = (1, 1);
        assert_eq!(has_valid_path(&grid, start, target), true);
    }

    #[test]
    fn get_valid_moves() {
        let grid = load_map(aoc::sample());
        let moves = determine_possible_moves(&grid, (2, 5));
        let mut hallway_locations = vec![(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];
        hallway_locations.sort();
        assert_eq!(moves, hallway_locations);
    }

    #[test]
    fn calculate_move_cost() {
        assert_eq!(determine_move_cost(&'A', (1, 1), (2, 3)), 3);
    }

    #[test]
    fn can_solve() {
        let grid = load_map(aoc::read_file("extra_inputs/almost_solved.txt"));
        let candidates = determine_eligible_move_candidates(&grid);
        assert_eq!(candidates, vec![(1, 10)]);
        let moves = determine_possible_moves(&grid, (1, 10));
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
        assert_eq!(solve_part_1(aoc::sample()), 11332);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
