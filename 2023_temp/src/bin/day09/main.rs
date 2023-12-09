use std::io::{self, BufRead};

fn extrapolate(sequence: &[i32]) -> (i32, i32) {
    let mut subsequences: Vec<Vec<i32>> = vec![sequence.to_vec()];

    let mut i = 0;
    while !subsequences.last().unwrap().iter().all(|x| x == &0) {
        let mut subsequence = vec![];
        let current = &subsequences[i];

        for i in 0..current.len() - 1 {
            subsequence.push(current[i + 1] - current[i])
        }
        subsequences.push(subsequence);
        i += 1;
    }

    subsequences.iter().rev().fold((0, 0), |(l, r), e| {
        (e.first().unwrap() - l, r + e.last().unwrap())
    })
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (answer2, answer1) = input
        .iter()
        .map(|x| extrapolate(x))
        .fold((0, 0), |(l1, r1), (l2, r2)| (l1 + l2, r1 + r2));

    println!("{answer1}\n{answer2}");
}
