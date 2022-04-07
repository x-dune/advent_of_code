use std::i32::MAX;

use aoc2021::util;

fn triangle_sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn main() {
    let mut input = util::input_as_ints_from_list(include_str!("input.txt"));
    input.sort_unstable();

    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    let mut answer1: i32 = MAX;
    let mut answer2: i32 = MAX;

    for new_pos in *min..=*max {
        let (possible_answer1, possible_answer2) = input
            .iter()
            .map(|horizontal_pos| {
                let diff = (horizontal_pos - new_pos).abs();
                (diff, triangle_sum(diff))
            })
            .fold((0, 0), |acc, curr| (acc.0 + curr.0, acc.1 + curr.1));

        if possible_answer1 < answer1 {
            answer1 = possible_answer1;
        }

        if possible_answer2 < answer2 {
            answer2 = possible_answer2;
        }
    }

    println!("{} {}", answer1, answer2)
}
