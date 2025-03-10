use std::{
    cmp::Reverse,
    collections::HashMap,
    io::{self, BufRead},
};

fn parse_input(s: &[String]) -> Vec<Vec<i64>> {
    s.iter()
        .map(|x| {
            x.split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let parsed = parse_input(&input);

    let answer1 = parsed.iter().fold(0, |a, b| {
        let is_asc = b[1] - b[0] > 0;
        let mut sorted = b.clone();
        if is_asc {
            sorted.sort();
        } else {
            sorted.sort_by_key(|x| Reverse(*x));
        }

        let mut safe = true;
        for i in 0..b.len() - 1 {
            if b[i] != sorted[i] {
                safe = false;
                break;
            }
            let diff = (b[i] - b[i + 1]).abs();
            if diff == 0 || diff > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            a + 1
        } else {
            a
        }
    });

    println!("{}\n{}", answer1, answer1);
}
