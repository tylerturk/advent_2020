fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Debug)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn new(line: &str) -> Self {
        // Player 1 starting position: 4
        // Player 2 starting position: 8
        let line_iter = line.split_whitespace();
        let pos = line_iter.last().unwrap().parse::<usize>().unwrap();
        Self { pos, score: 0 }
    }

    fn do_move(&mut self, spaces: usize) {
        self.pos = self.pos + spaces;
        if self.pos > 10 {
            self.pos = self.pos % 10;
        }
        if self.pos == 0 {
            self.pos = 10;
        }
        self.score += self.pos;
    }
}

struct Dice {
    last_val: usize,
    rolls: usize,
}

impl Dice {
    fn new() -> Self {
        Self {
            last_val: 0,
            rolls: 0,
        }
    }
    fn practice_rolls(&mut self) -> usize {
        let mut val: usize = 0;
        for _ in 1..=3 {
            let mut next_val = self.last_val + 1;
            if next_val > 100 {
                next_val -= 100;
            }
            val += next_val;
            self.last_val = next_val;
        }
        self.rolls += 3;
        val
    }
}

fn solve_part_1(contents: String) -> usize {
    let mut players: Vec<Player> = Vec::new();
    for line in contents.lines() {
        players.push(Player::new(line));
    }
    let mut d = Dice::new();
    let mut has_winner = false;
    loop {
        for player in players.iter_mut() {
            player.do_move(d.practice_rolls());
            if player.score >= 1000 {
                has_winner = true;
                break;
            }
        }
        if has_winner {
            break;
        }
    }
    players.iter().map(|p| p.score).min().unwrap() * d.rolls
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 739785);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
