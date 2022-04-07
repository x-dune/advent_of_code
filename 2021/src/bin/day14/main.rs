use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn polymer_pair_count(polymer: &str) -> HashMap<(char, char), u64> {
    let polymer_bytes = polymer.as_bytes();
    let mut pattern_count = HashMap::new();
    for i in 1..polymer_bytes.len() {
        *pattern_count
            .entry((polymer_bytes[i - 1] as char, polymer_bytes[i] as char))
            .or_insert(0) += 1;
    }
    pattern_count
}

fn next_polymer_pair_count(
    pattern_count: &HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut next_pattern_count = HashMap::new();

    for (pattern @ &(char1, char2), &count) in pattern_count {
        if let Some(&to_insert) = rules.get(pattern) {
            *next_pattern_count.entry((char1, to_insert)).or_insert(0) += count;
            *next_pattern_count.entry((to_insert, char2)).or_insert(0) += count;
        } else {
            *next_pattern_count.entry(*pattern).or_insert(0) += count;
        }
    }
    next_pattern_count
}

fn answer(pattern_count: &HashMap<(char, char), u64>, last_element: char) -> u64 {
    let mut element_count = HashMap::new();

    for ((char1, _), count) in pattern_count {
        // Only add the first item in pair, since each individual element occurs in two pairs
        // Except the asbsolute first and last element, the first element is accounted for here
        *element_count.entry(*char1).or_insert(0) += count;
    }
    // Account for absolute last element
    *element_count.entry(last_element).or_insert(0) += 1;

    let (max, min) = element_count.iter().fold((0, u64::MAX), |acc, (_, &curr)| {
        (max(acc.0, curr), min(acc.1, curr))
    });

    max - min
}

fn main() {
    let (polymer_template, rules_raw) = include_str!("input.txt").split_once("\n\n").unwrap();
    let absolute_last_element = polymer_template.chars().last().unwrap();

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

    let mut pair_count = polymer_pair_count(polymer_template);
    for i in 1..=40 {
        pair_count = next_polymer_pair_count(&pair_count, &rules);
        if i == 10 {
            println!("part 1 {}", answer(&pair_count, absolute_last_element));
        }
    }
    println!("part 2 {}", answer(&pair_count, absolute_last_element));
}
