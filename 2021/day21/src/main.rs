use cached::proc_macro::cached;
use cached::UnboundCache;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Clone, Debug)]
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
        if spaces == 0 {
            return;
        }
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

#[cached]
fn dice_gen() -> Vec<usize> {
    let mut permutations = Vec::new();
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                permutations.push(d1 + d2 + d3);
            }
        }
    }
    permutations.sort();
    permutations
}

#[cached(
    type = "UnboundCache<String, (i64, i64)>",
    create = "{ UnboundCache::new() }",
    convert=r#"{format!("{}{}{}{}{}{}", p1.pos, p1.score, p2.pos, p2.score, spaces, p1s_move)}"#
)]
fn recurse_games(p1: &mut Player, p2: &mut Player, spaces: usize, p1s_move: bool) -> (i64, i64) {
    let win_score = 21;
    let mut p1_wins: i64 = 0;
    let mut p2_wins: i64 = 0;
    let permutations = dice_gen();
    if spaces != 0 {
        if p1s_move {
            p1.do_move(spaces);
            if p1.score >= win_score {
                return (1, 0);
            }
        } else {
            p2.do_move(spaces);
            if p2.score >= win_score {
                return (0, 1);
            }
        }
    }
    for m1 in permutations.iter() {
        let (p1_wins_rec, p2_wins_rec) = recurse_games(&mut p1.clone(), &mut p2.clone(), *m1, !p1s_move);
        p1_wins += p1_wins_rec;
        p2_wins += p2_wins_rec;
    }
    (p1_wins, p2_wins)
}

fn solve_part_2(contents: String) -> i64 {
    let mut players: Vec<Player> = Vec::new();
    for line in contents.lines() {
        players.push(Player::new(line));
    }
    let mut p2 = players.pop().unwrap();
    let mut p1 = players.pop().unwrap();
    let (p1_wins, p2_wins) = recurse_games(&mut p1, &mut p2, 0, false);

    if p1_wins > p2_wins {
        p1_wins
    } else {
        p2_wins
    }
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
        assert_eq!(solve_part_2(aoc::sample()), 444356092776315);
    }
}
