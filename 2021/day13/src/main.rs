use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2:");
    solve_part_2(aoc::input());
}

fn fold_paper(contents: String, one_fold: bool) -> HashMap<String, bool> {
    let mut papers: HashMap<String, bool> = HashMap::new();
    let mut coords = true;
    let mut folds: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            coords = false;
            continue;
        }
        if coords {
            papers.insert(line.to_string(), true);
        } else {
            let fold: String = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .last()
                .unwrap()
                .to_string();
            folds.push(fold);
        }
    }

    for fold in folds {
        let fold_bits: Vec<&str> = fold.split("=").collect();
        let fold_pos: usize = fold_bits.last().unwrap().parse::<usize>().unwrap();
        if fold_bits.first().unwrap() == &"x" {
            for key in papers.clone().keys() {
                let pos_bits: Vec<&str> = key.split(",").collect();
                let x = pos_bits.first().unwrap().parse::<usize>().unwrap();
                let y = pos_bits.last().unwrap().parse::<usize>().unwrap();
                if x == fold_pos {
                    papers.remove(key);
                } else if x > fold_pos {
                    let new_x = 2 * fold_pos - x;
                    papers.insert(format!("{},{}", new_x, y), true);
                    papers.remove(key);
                }
            }
        } else {
            for key in papers.clone().keys() {
                let pos_bits: Vec<&str> = key.split(",").collect();
                let x = pos_bits.first().unwrap().parse::<usize>().unwrap();
                let y = pos_bits.last().unwrap().parse::<usize>().unwrap();
                if y == fold_pos {
                    papers.remove(key);
                } else if y > fold_pos {
                    let new_y = 2 * fold_pos - y;
                    papers.insert(format!("{},{}", x, new_y), true);
                    papers.remove(key);
                }
            }
        }
        if one_fold {
            break;
        }
    }
    papers
}

fn solve_part_1(contents: String) -> usize {
    let papers = fold_paper(contents, true);
    papers.keys().len()
}

fn solve_part_2(contents: String) {
    let papers = fold_paper(contents, false);
    let mut output: Vec<Vec<bool>> = Vec::new();

    for _ in 0..6 {
        output.push(vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ]);
    }
    for key in papers.keys() {
        let pos_bits: Vec<&str> = key.split(",").collect();
        let x = pos_bits.first().unwrap().parse::<usize>().unwrap();
        let y = pos_bits.last().unwrap().parse::<usize>().unwrap();
        output[y][x] = true;
    }

    for row in output {
        for col in row {
            if col {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 17);
    }
}
