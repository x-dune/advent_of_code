use std::{
    cmp::{max, min},
    collections::HashMap,
    u64::MAX,
};

// NNCB
// NN NC NB

// NCNBCHB
// NC CN NB BC CH HB

// pattern is two letter polymer
fn get_pattern_count(polymer: &str) -> HashMap<(char, char), u64> {
    let polymer_bytes = polymer.as_bytes();
    let mut pattern_count = HashMap::new();
    for i in 1..polymer_bytes.len() {
        *pattern_count
            .entry((polymer_bytes[i - 1] as char, polymer_bytes[i] as char))
            .or_insert(0) += 1;
    }

    pattern_count
}

fn next_pattern_count(
    pattern_count: &HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut next_pattern_count = HashMap::new();

    for (pattern @ (char1, char2), &count) in pattern_count {
        if let Some(&to_insert) = rules.get(&pattern) {
            *next_pattern_count.entry((*char1, to_insert)).or_insert(0) += count;
            *next_pattern_count.entry((to_insert, *char2)).or_insert(0) += count;
        } else {
            *next_pattern_count.entry(*pattern).or_insert(0) += count;
        }
    }
    next_pattern_count
}

// unrvel pattern into occurence of single char, sum them, get max and min occurence, return diff
fn get_answer(pattern_count: &HashMap<(char, char), u64>) -> u64 {
    let mut char_count = HashMap::new();

    for ((char1, char2), count) in pattern_count {
        let count = *count as f64;
        *char_count.entry(*char1).or_insert(0.) += count / 2.;
        *char_count.entry(*char2).or_insert(0.) += count / 2.;
    }
    let char_count_normalized = char_count
        .into_iter()
        // need to ceil because terminal patterns won't have char occur twice
        .map(|(c, n)| (c, n.ceil() as u64))
        .collect::<HashMap<char, u64>>();

    let (max, min) = char_count_normalized
        .iter()
        .fold((0, MAX), |acc, (_, curr)| {
            (max(acc.0, *curr), min(acc.1, *curr))
        });

    max - min
}

fn main() {
    let (polymer_template, rules_raw) = include_str!("input.txt").split_once("\n\n").unwrap();

    let rules = rules_raw
        .lines()
        .map(|x| {
            let (pattern, to_insert) = x.split_once(" -> ").unwrap();
            (
                (pattern.as_bytes()[0] as char, pattern.as_bytes()[1] as char),
                to_insert.as_bytes()[0] as char,
            )
        })
        .collect::<HashMap<(char, char), char>>();

    let mut pattern_count = get_pattern_count(polymer_template);

    for i in 1..=40 {
        pattern_count = next_pattern_count(&pattern_count, &rules);
        if i == 10 {
            println!("part 1 {}", get_answer(&pattern_count));
        }
    }
    println!("part 2 {}", get_answer(&pattern_count));
}
