use std::io::{self, BufRead};

fn parse_input_1(s: &str) -> Vec<i64> {
    s.split(' ')
        .filter_map(|x| x.trim().parse::<i64>().ok())
        .collect::<Vec<_>>()
}

fn parse_input_2(s: &str) -> i64 {
    let result = s
        .split(' ')
        .filter_map(|x| {
            let trimmed = x.trim();
            if trimmed.parse::<i64>().is_ok() {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    result
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let times = parse_input_1(&input[0]);
    let dists = parse_input_1(&input[1]);

    let time_part2 = parse_input_2(&input[0]);
    let dist_part2 = parse_input_2(&input[1]);

    let mut possible_win = vec![];
    for (i, time) in times.iter().enumerate() {
        possible_win.push(0);
        for t in 1..*time {
            if t * (time - t) > dists[i] {
                possible_win[i] += 1;
            }
        }
    }

    let mut answer2 = 0;
    for t in 1..time_part2 {
        if t * (time_part2 - t) > dist_part2 {
            answer2 += 1;
        }
    }
    let answer1 = possible_win.iter().product::<i32>();
    println!("{}\n{}", answer1, answer2);
}
