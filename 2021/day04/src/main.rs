use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Clone)]
struct BingoBoard {
    spots: Vec<Vec<BingoSpot>>,
    cols_marked: HashMap<usize, i32>,
    rows_marked: HashMap<usize, i32>,
    values: Vec<i32>,
}

impl BingoBoard {
    fn mark(&mut self, num_to_mark: i32) {
        let spots = self.spots.clone();
        if self.values.contains(&num_to_mark) {
            for (row_ind, row) in spots.iter().enumerate() {
                for (col_ind, col) in row.iter().enumerate() {
                    if col.value == num_to_mark {
                        self.spots
                            .get_mut(row_ind)
                            .unwrap()
                            .get_mut(col_ind)
                            .unwrap()
                            .mark();
                        *self.cols_marked.entry(col_ind).or_insert(0) += 1;
                        *self.rows_marked.entry(row_ind).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    fn check_winning(&self) -> bool {
        for (_, v) in self.cols_marked.iter() {
            if v == &5 {
                return true;
            }
        }
        for (_, v) in self.rows_marked.iter() {
            if v == &5 {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum: i32 = 0;
        for row in self.spots.iter() {
            for col in row.iter() {
                if !col.is_marked() {
                    sum += col.value;
                }
            }
        }
        sum
    }
}

#[derive(Clone, Copy, Debug)]
struct BingoSpot {
    value: i32,
    marked: bool,
}

impl BingoSpot {
    fn mark(&mut self) {
        self.marked = true;
    }

    fn is_marked(self) -> bool {
        self.marked
    }
}

fn parse_bingo_input<'content>(contents: String) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut c = contents.lines();
    let called_numbers: Vec<i32> = c
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board: Vec<Vec<BingoSpot>> = Vec::new();
    let mut values: Vec<i32> = Vec::new();

    for r in c {
        if r == "" {
            continue;
        }
        let mut row: Vec<BingoSpot> = Vec::new();
        let row_numbers: Vec<i32> = r
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        values.append(&mut row_numbers.clone());
        for row_number in row_numbers {
            row.push(BingoSpot {
                marked: false,
                value: row_number,
            });
        }
        board.push(row);
        if board.len() == 5 {
            let new_board = board.to_vec();
            let new_values = values.to_vec();
            boards.push(BingoBoard {
                cols_marked: HashMap::new(),
                rows_marked: HashMap::new(),
                spots: new_board.clone(),
                values: new_values.clone(),
            });
            board.clear();
            values.clear();
        }
    }
    (called_numbers, boards)
}

fn solve_part_1(contents: String) -> i32 {
    let (numbers, mut boards) = parse_bingo_input(contents);
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.check_winning() {
                return board.sum_unmarked() * number;
            }
        }
    }
    0
}

fn solve_part_2(contents: String) -> i32 {
    let (numbers, mut boards) = parse_bingo_input(contents);
    let mut last_board: Option<BingoBoard> = None;
    let mut last_number_called: Option<i32> = None;
    for number in numbers {
        let mut ind_to_remove: Vec<usize> = Vec::new();
        for (board_ind, board) in boards.iter_mut().enumerate() {
            board.mark(number);
            let winning: bool = board.check_winning();
            if winning {
                last_board = Some(board.clone());
                last_number_called = Some(number.clone());
                ind_to_remove.push(board_ind);
            }
        }
        for ind in ind_to_remove.iter().rev() {
            boards.remove(*ind);
        }
    }
    match last_board {
        Some(board) => board.sum_unmarked() * last_number_called.unwrap(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 4512);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 1924);
    }
}
