fn main() {
    println!("Part 1: {}", solve_part_1(aoc::input()));
    println!("Part 2: {}", solve_part_2(aoc::input()));
}

#[derive(Clone, Debug, PartialEq)]
enum SnailNumber {
    SnailNumberStruct {
        depth: usize,
        left: Box<SnailNumber>,
        right: Box<SnailNumber>,
    },
    Value(i64),
    None,
}

impl SnailNumber {
    fn explode(&mut self) {
        match &self {
            SnailNumber::SnailNumberStruct { depth, left, right } => {
                println!("lulz");
                if depth == &3 {
                    match &**left {
                        SnailNumber::SnailNumberStruct { right, ..} => {
                            println!("Should add {:?}", right);
                        }
                        _ => {}
                    }
                } else {
                    left.explode();
                }
            }
            _ => {}
        }
    }
}

fn solve_part_1(contents: String) -> i64 {
    todo!();
}

fn solve_part_2(contents: String) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_explode() {
        // [[[[[9,8],1],2],3],4]
        let mut sample = SnailNumber::SnailNumberStruct {
            depth: 0,
            left: Box::new(SnailNumber::SnailNumberStruct {
                depth: 1,
                left: Box::new(SnailNumber::SnailNumberStruct {
                    depth: 2,
                    left: Box::new(SnailNumber::SnailNumberStruct {
                        depth: 3,
                        left: Box::new(SnailNumber::SnailNumberStruct {
                            depth: 4,
                            left: Box::new(SnailNumber::Value(9)),
                            right: Box::new(SnailNumber::Value(8)),
                        }),
                        right: Box::new(SnailNumber::Value(1)),
                    }),
                    right: Box::new(SnailNumber::Value(2)),
                }),
                right: Box::new(SnailNumber::Value(3)),
            }),
            right: Box::new(SnailNumber::Value(4)),
        };
        println!("{:?}", sample);
        sample.explode();
        println!("{:?}", sample);
        todo!();
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
