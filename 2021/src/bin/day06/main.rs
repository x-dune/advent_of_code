use std::collections::HashMap;

fn parse_input() -> Vec<i64> {
    return include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
}

fn solve(input: &Vec<i64>, number_of_days: i64) -> i64 {
    let mut fish_to_days_left = HashMap::new();

    for days_left in input {
        *fish_to_days_left.entry(*days_left).or_insert(0) += 1;
    }

    for _ in 0..number_of_days {
        let mut next_fish_to_days_left = HashMap::new();
        for days_left in 0..=8 {
            if let Some(&current) = fish_to_days_left.get(&days_left) {
                if current != 0 {
                    if days_left == 0 {
                        *next_fish_to_days_left.entry(8).or_insert(0) += current;
                        *next_fish_to_days_left.entry(6).or_insert(0) += current;
                    } else {
                        *next_fish_to_days_left.entry(days_left - 1).or_insert(0) += current;
                    }
                }
            }
        }
        fish_to_days_left = next_fish_to_days_left;
    }

    return fish_to_days_left.values().sum();
}

fn solution1() -> i64 {
    let input = parse_input();
    return solve(&input, 80);
}

fn solution2() -> i64 {
    let input = parse_input();
    return solve(&input, 256);
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
