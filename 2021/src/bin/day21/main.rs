use std::collections::BTreeMap;

fn parse_input() -> [i64; 2] {
    let (player1, player2) = include_str!("input.txt").trim().split_once("\n").unwrap();
    [player1, player2].map(|a| a.split_once(": ").unwrap().1.parse::<i64>().unwrap())
}

fn cycle(limit: i64, n: i64) -> i64 {
    ((n - 1) % limit) + 1
}

fn solution1(start_positions: [i64; 2]) -> i64 {
    let mut positions = start_positions;
    let mut scores = [0, 0];
    let mut prev_roll = 0;

    for i in 0.. {
        let player_index = i % 2;
        let rolls = [prev_roll + 1, prev_roll + 2, prev_roll + 3].map(|a| cycle(100, a));
        let to_move: i64 = rolls.iter().sum();

        positions[player_index] = cycle(10, positions[player_index] + to_move);
        scores[player_index] = scores[player_index] + positions[player_index];
        prev_roll = *rolls.last().unwrap();

        if scores.iter().any(|&a| a >= 1000) {
            return scores.iter().min().unwrap() * (i as i64 + 1) * 3;
        }
    }
    unreachable!()
}

fn solution2(start_positions: [i64; 2]) -> i64 {
    // key: pos1 pos2 score1 score2, val: count
    let mut universes = BTreeMap::from([([start_positions[0], start_positions[1], 0, 0], 1i64)]);
    let mut universes_won = [0, 0];

    let quantum_rolls = (1..=3)
        .flat_map(|roll1| {
            (1..=3).flat_map(move |roll2| (1..=3).map(move |roll3| (roll1 + roll2 + roll3)))
        })
        .collect::<Vec<_>>();

    while !universes.is_empty() {
        for i in 0..2 {
            let player_index = i % 2;

            let mut next = BTreeMap::new();
            for (universe, count) in &universes {
                let pos = universe[player_index];
                let score = universe[player_index + 2];
                for next_pos in quantum_rolls.iter().map(|roll| cycle(10, pos + roll)) {
                    let next_score = score + next_pos;

                    if next_score >= 21 {
                        universes_won[player_index] += count;
                    } else {
                        let mut next_universe = universe.clone();
                        next_universe[player_index] = next_pos;
                        next_universe[player_index + 2] = next_score;

                        *next.entry(next_universe).or_insert(0) += count;
                    }
                }
            }
            universes = next;
        }
    }

    return *universes_won.iter().max().unwrap();
}
fn main() {
    let start_positions = parse_input();
    println!("{}", solution1(start_positions));
    println!("{}", solution2(start_positions));
}
