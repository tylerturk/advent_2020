use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

fn solve_part_1(contents: String) -> i32 {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut height: usize = 0;
    let mut low_points: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let row: Vec<i32> = line
            .chars()
            .map(|x| String::from(x).parse::<i32>().unwrap())
            .collect();
        height += 1;
        map.push(row);
    }

    for (ci, col) in map.iter().enumerate() {
        for (ri, _) in col.iter().enumerate() {
            let mut low_point = true;
            let cur_value = map.get(ci).unwrap().get(ri).unwrap();
            if ci > 0 {
                if cur_value >= map.get(ci - 1).unwrap().get(ri).unwrap() {
                    low_point = false;
                }
            }
            if ci <= height - 2 {
                if cur_value >= map.get(ci + 1).unwrap().get(ri).unwrap() {
                    low_point = false;
                }
            }
            if ri > 0 {
                if cur_value >= map.get(ci).unwrap().get(ri - 1).unwrap() {
                    low_point = false;
                }
            }
            if ri < map.get(ci).unwrap().len() - 1 {
                if cur_value >= map.get(ci).unwrap().get(ri + 1).unwrap() {
                    low_point = false;
                }
            }
            if low_point {
                low_points.push(*cur_value);
            }
        }
    }
    low_points.iter().sum::<i32>() + low_points.len() as i32
}

struct Map {
    map: HashMap<String, i32>,
}

fn solve_part_2(contents: String) -> usize {
    // let mut map: HashMap<String, i32> = HashMap::new();
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut basins: HashMap<String, usize> = HashMap::new();
    for line in contents.lines() {
        let row: Vec<i32> = line
            .chars()
            .map(|x| String::from(x).parse::<i32>().unwrap())
            .collect();
        if height == 0 {
            width = row.len() - 1;
        }
        height += 1;
        map.push(row);
    }
    // for line in contents.lines() {
    //     for (y, val) in line
    //         .chars()
    //         .map(|x| String::from(x).parse::<i32>().unwrap())
    //         .enumerate()
    //     {
    //         map.insert(format!("{},{}", xi, y), val);
    //         if xi == 0 {
    //             yi += 1;
    //         }
    //     }
    //     xi += 1;
    // }
    for (ci, col) in map.iter().enumerate() {
        for (ri, _) in col.iter().enumerate() {
            let mut low_point = true;
            let cur_value = map.get(ci).unwrap().get(ri).unwrap();
            if ci > 0 {
                if cur_value >= map.get(ci - 1).unwrap().get(ri).unwrap() {
                    low_point = false;
                }
            }
            if ci <= height - 2 {
                if cur_value >= map.get(ci + 1).unwrap().get(ri).unwrap() {
                    low_point = false;
                }
            }
            if ri > 0 {
                if cur_value >= map.get(ci).unwrap().get(ri - 1).unwrap() {
                    low_point = false;
                }
            }
            if ri < map.get(ci).unwrap().len() - 1 {
                if cur_value >= map.get(ci).unwrap().get(ri + 1).unwrap() {
                    low_point = false;
                }
            }
            if low_point {
                let mut count = solve_lowpoint_2(map.clone(), ri, ci, *cur_value, vec![]);
                count.sort();
                count.dedup();
                basins.insert(format!("{},{}", ri, ci), count.len());
            }
        }
    }
    let mut max_basins: Vec<&usize> = basins.values().into_iter().collect();
    max_basins.sort_by(|a, b| b.cmp(a));
    max_basins.into_iter().take(3).product()
}

#[cached]
fn solve_lowpoint_2(
    map: Vec<Vec<i32>>,
    x: usize,
    y: usize,
    cur_value: i32,
    in_basin: Vec<String>,
) -> Vec<String> {
    let mut in_basin = in_basin;
    let height: usize = map.len();
    let width: usize = map.clone().get(0).unwrap().len() - 2;
    let pos = format!("{},{}", x, y);
    in_basin.push(pos.clone());
    if x > 0 {
        let adjacent_value: i32 = *map.clone().get(y).unwrap().get(x - 1).unwrap();
        if cur_value < adjacent_value && adjacent_value != 9 {
            in_basin.push(format!("{},{}", x - 1, y));
            in_basin = solve_lowpoint_2(map.clone(), x - 1, y, adjacent_value, in_basin.clone());
        }
    }
    if x <= width {
        let adjacent_value: i32 = *map.clone().get(y).unwrap().get(x + 1).unwrap();
        if cur_value < adjacent_value && adjacent_value != 9 {
            in_basin.push(format!("{},{}", x + 1, y));
            in_basin = solve_lowpoint_2(map.clone(), x + 1, y, adjacent_value, in_basin.clone());
        }
    }
    if y > 0 {
        let adjacent_value: i32 = *map.clone().get(y - 1).unwrap().get(x).unwrap();
        if cur_value < adjacent_value && adjacent_value != 9 {
            in_basin.push(format!("{},{}", x, y - 1));
            in_basin = solve_lowpoint_2(map.clone(), x, y - 1, adjacent_value, in_basin.clone());
        }
    }
    if y < height - 1 {
        let adjacent_value: i32 = *map.clone().get(y + 1).unwrap().get(x).unwrap();
        if cur_value < adjacent_value && adjacent_value != 9 {
            in_basin.push(format!("{},{}", x, y + 1));
            in_basin = solve_lowpoint_2(map.clone(), x, y + 1, adjacent_value, in_basin.clone());
        }
    }
    in_basin.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 15);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 1134);
    }
}
