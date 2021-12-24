use std::cmp::{max, min};

const INST_SET_LENGTH: usize = 18;

fn parse_input() -> Vec<Vec<String>> {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();

    let mut instruction_sets = vec![];

    for i in 0..(instructions.len() / INST_SET_LENGTH) {
        instruction_sets
            .push(instructions[(i * INST_SET_LENGTH)..(i + 1) * INST_SET_LENGTH].to_vec())
    }

    instruction_sets
}

fn to_14_digit_place(num: i64, place: u32) -> i64 {
    num * 10i64.pow(13 - place)
}

fn highest_lowest_monad(pair: &Vec<(usize, usize, i64)>) -> (i64, i64) {
    let mut highest_monad = 0;
    let mut lowest_monad = 0;

    for &(i, j, arg) in pair {
        // higher_digit is a more significant digit; i.e. 1000s is more significant thatn 100s
        let higher_digit = min(i, j) as u32;
        let lower_digit = max(i, j) as u32;

        if arg.is_positive() {
            highest_monad += to_14_digit_place(9 - arg, higher_digit);
            highest_monad += to_14_digit_place(9, lower_digit);

            lowest_monad += to_14_digit_place(1, higher_digit);
            lowest_monad += to_14_digit_place(1 + arg, lower_digit);
        } else {
            highest_monad += to_14_digit_place(9, higher_digit);
            highest_monad += to_14_digit_place(9 - arg.abs(), lower_digit);

            lowest_monad += to_14_digit_place(1 + arg.abs(), higher_digit);
            lowest_monad += to_14_digit_place(1, lower_digit);
        }
    }

    (highest_monad, lowest_monad)
}

fn main() {
    let insts = parse_input();

    let mut pairs = vec![];
    let mut stack = vec![];
    for (i, inst_set) in insts.iter().enumerate() {
        match inst_set[4].as_str() {
            "div z 1" => {
                // push to stack
                let arg = inst_set[15]
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                stack.push((i, arg))
            }
            "div z 26" => {
                // pop from stack
                let arg = inst_set[5]
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                let (j, other_arg) = stack.pop().unwrap();
                pairs.push((i, j, arg + other_arg));
            }
            _ => unreachable!(),
        }
    }

    let (highest, lowest) = highest_lowest_monad(&pairs);
    println!("{}\n{}", highest, lowest);
}
