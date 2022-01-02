use cached::proc_macro::cached;
use cached::UnboundCache;
use std::fmt;

fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
    variables: Vec<String>,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let mut bits = input.split_whitespace();
        let op = bits.next().unwrap();
        let operation = match op {
            "inp" => Operation::INP,
            "add" => Operation::ADD,
            "mul" => Operation::MUL,
            "div" => Operation::DIV,
            "mod" => Operation::MOD,
            "eql" => Operation::EQL,
            _ => panic!("Found invalid operation"),
        };
        let mut variables: Vec<String> = Vec::new();
        for bit in bits {
            variables.push(bit.to_string());
        }
        Self {
            operation,
            variables,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.operation {
            Operation::INP => "inp",
            Operation::ADD => "add",
            Operation::MUL => "mul",
            Operation::DIV => "div",
            Operation::MOD => "mod",
            Operation::EQL => "eql",
        };
        write!(f, "{} {}", op, self.variables.join(" "))
    }
}

#[cached(
    type = "UnboundCache<String, Option<i64>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{format!("w{}z{}instlen{}", w, z, instructions.len())}"#
)]
fn recurse_solve(
    w: i64,
    z: i64,
    instructions: &Vec<Instruction>,
    cur_num: i64,
    reverse: bool,
) -> Option<i64> {
    let mut w = w;
    let mut x = 0;
    let mut y = 0;
    let mut z = z;
    for (ind, instruction) in instructions.clone().iter().enumerate() {
        match instruction.operation {
            Operation::INP => {
                let r = 1..=9;
                let new_instructions = instructions[ind + 1..].to_vec();
                if reverse {
                    for num in r.rev() {
                        let resp = recurse_solve(
                            num,
                            z,
                            &new_instructions.clone(),
                            format!("{}{}", cur_num, num).parse::<i64>().unwrap(),
                            reverse,
                        );
                        if resp.is_some() {
                            return resp;
                        }
                    }
                } else {
                    for num in r {
                        let resp = recurse_solve(
                            num,
                            z,
                            &new_instructions.clone(),
                            format!("{}{}", cur_num, num).parse::<i64>().unwrap(),
                            reverse,
                        );
                        if resp.is_some() {
                            return resp;
                        }
                    }
                }
            }
            Operation::ADD => {
                let val1 = match instruction.variables[0].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => panic!("Invalid variable"),
                };
                let val2 = match instruction.variables[1].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => instruction.variables[1].parse::<i64>().unwrap(),
                };
                let new_val = val1 + val2;
                match instruction.variables[0].as_str() {
                    "w" => w = new_val,
                    "x" => x = new_val,
                    "y" => y = new_val,
                    "z" => z = new_val,
                    _ => panic!("Invalid variable"),
                };
            }
            Operation::MUL => {
                let val1 = match instruction.variables[0].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => panic!("Invalid variable"),
                };
                let val2 = match instruction.variables[1].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => instruction.variables[1].parse::<i64>().unwrap(),
                };
                let new_val = val1 * val2;
                match instruction.variables[0].as_str() {
                    "w" => w = new_val,
                    "x" => x = new_val,
                    "y" => y = new_val,
                    "z" => z = new_val,
                    _ => panic!("Invalid variable"),
                };
            }
            Operation::DIV => {
                let val1 = match instruction.variables[0].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => panic!("Invalid variable"),
                };
                let val2 = match instruction.variables[1].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => instruction.variables[1].parse::<i64>().unwrap(),
                };
                if val2 == 0 {
                    println!("WARNING: Trying to divide by zero");
                    continue;
                }
                let new_val = val1 / val2;
                match instruction.variables[0].as_str() {
                    "w" => w = new_val,
                    "x" => x = new_val,
                    "y" => y = new_val,
                    "z" => z = new_val,
                    _ => panic!("Invalid variable"),
                };
            }
            Operation::MOD => {
                let val1 = match instruction.variables[0].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => panic!("Invalid variable"),
                };
                let val2 = match instruction.variables[1].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => instruction.variables[1].parse::<i64>().unwrap(),
                };
                let new_val = val1 % val2;
                match instruction.variables[0].as_str() {
                    "w" => w = new_val,
                    "x" => x = new_val,
                    "y" => y = new_val,
                    "z" => z = new_val,
                    _ => panic!("Invalid variable"),
                };
            }
            Operation::EQL => {
                let val1 = match instruction.variables[0].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => panic!("Invalid variable"),
                };
                let val2 = match instruction.variables[1].as_str() {
                    "w" => w,
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    _ => instruction.variables[1].parse::<i64>().unwrap(),
                };
                let mut new_val = 0;
                if val1 == val2 {
                    new_val = 1;
                }
                match instruction.variables[0].as_str() {
                    "w" => w = new_val,
                    "x" => x = new_val,
                    "y" => y = new_val,
                    "z" => z = new_val,
                    _ => panic!("Invalid variable"),
                };
            }
        }
        if ind == instructions.len() - 1 {
            if z == 0 && cur_num > 10000000000000 {
                return Some(cur_num);
            }
            return None;
        }
    }
    None
}

fn solve_part_1(contents: String) -> i64 {
    let instructions = contents
        .lines()
        .map(|x| Instruction::new(x))
        .collect::<Vec<Instruction>>();
    let val = recurse_solve(0, 0, &instructions, 0, true);
    val.unwrap_or(0)
}

fn solve_part_2(contents: String) -> i64 {
    let instructions = contents
        .lines()
        .map(|x| Instruction::new(x))
        .collect::<Vec<Instruction>>();
    let val = recurse_solve(0, 0, &instructions, 0, false);
    val.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(aoc::sample()), 0);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(aoc::sample()), 0);
    }
}
