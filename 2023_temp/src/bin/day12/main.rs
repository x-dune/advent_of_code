use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn parse_input(input: &[String], times: i32) -> Vec<(Vec<char>, Vec<i32>)> {
    input
        .iter()
        .map(|e| {
            let (spring, broken) = e.split_once(' ').unwrap();
            let broken = broken
                .split(',')
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let mut unfolded_spring = spring.to_owned();
            let mut unfolded_broken = broken.clone();
            for _ in 0..times - 1 {
                unfolded_spring.push_str(&format!("?{}", spring));
                unfolded_broken.extend(broken.iter().clone());
            }
            (unfolded_spring.chars().collect(), unfolded_broken)
        })
        .collect()
}

// position in initial, # of broken spring in current continguous group, amount of broken contiguous groups done
type CacheState = (usize, i32, usize);

fn gen_dp(
    initial: &[char],
    group: &Vec<i32>,
    i: usize,
    current_group: i32,
    group_done: usize,
    cache: &mut HashMap<CacheState, i64>,
) -> i64 {
    let cache_key = (i, current_group, group_done);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    // base case
    if i == initial.len() {
        if group_done == group.len() && current_group == 0
            || group_done + 1 == group.len() && current_group == group[group_done]
        {
            return 1;
        } else {
            return 0;
        }
    }

    let mut result = 0;
    for c in ['#', '.'] {
        if initial[i] == '?' || initial[i] == c {
            if c == '#' {
                result += gen_dp(initial, group, i + 1, current_group + 1, group_done, cache);
            } else if current_group == 0 {
                // case '.'
                result += gen_dp(initial, group, i + 1, current_group, group_done, cache);
            } else if group_done < group.len() && current_group == group[group_done] {
                // case '.'
                result += gen_dp(initial, group, i + 1, 0, group_done + 1, cache);
            }
        }
    }
    cache.insert(cache_key, result);
    result
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let answer1 = parse_input(&input, 1)
        .iter()
        .map(|e| gen_dp(&e.0, &e.1, 0, 0, 0, &mut HashMap::new()))
        .sum::<i64>();
    let answer2 = parse_input(&input, 5)
        .iter()
        .map(|e| gen_dp(&e.0, &e.1, 0, 0, 0, &mut HashMap::new()))
        .sum::<i64>();

    println!("{}\n{}", answer1, answer2);
}
