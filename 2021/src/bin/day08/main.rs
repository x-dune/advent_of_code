use std::collections::HashMap;

use aoc2021::util;

// truths
// 1 - 2 seg /
// 4 - 4 seg /
// 7 - 3 seg /
// 8  - 7 seg /

// assumptions fives
// 2 -
// 3 - same 2 digits as 1 with extra 3
// 5 -

// assumptions sixs
// 9
// 6
// 0

////
///
///
// f - 9

// a - 8 -> in 7 and 8 but not 1

// c - 8

// d - 7 -> in 8 and 4 but not in 1 and 7
// g - 7

// b - 6
// e - 4

fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
    let lines = util::input_as_lines(include_str!("input.txt"));

    lines
        .iter()
        .map(|s| {
            let splitted = s.split(" | ").collect::<Vec<&str>>();
            let patterns = splitted[0]
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect();
            let output = splitted[1]
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect();

            (patterns, output)
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

fn find_seq(patterns: &Vec<String>, length: usize) -> Vec<char> {
    patterns
        .iter()
        .find(|x| x.len() == length)
        .unwrap()
        .chars()
        .collect()
}

fn get_translation(pattern: &str, cypher_map: &HashMap<char, char>) -> String {
    let mut output = vec![];
    for i in pattern.chars() {
        output.push(cypher_map[&i]);
    }

    output.sort();
    output.iter().collect()
}

fn decode_output(tranlation_map: &HashMap<String, String>, pattern: &str) -> String {
    let mut vec = pattern.chars().collect::<Vec<char>>();
    vec.sort();
    let normalized = vec.iter().collect::<String>();
    return tranlation_map[&normalized].to_owned();
}

fn main() {
    let input = parse_input();

    let mut answer1 = 0;
    for (_, output) in &input {
        for o in output {
            let len = o.len();
            if [2, 3, 4, 7].contains(&len) {
                answer1 += 1;
            }
        }
    }
    println!("{}", answer1);

    // actual to mixed
    let mut answer2 = 0;
    for (patterns, output) in &input {
        let mut cypher_map: HashMap<char, char> = HashMap::new();

        let seq_1 = find_seq(&patterns, 2);
        let seq_4 = find_seq(&patterns, 4);
        let seq_7 = find_seq(&patterns, 3);
        let seq_8 = find_seq(&patterns, 7);

        let count: HashMap<char, i32> = patterns.iter().fold(HashMap::new(), |mut acc, curr| {
            for char in curr.chars() {
                *acc.entry(char).or_insert(0) += 1;
            }
            acc
        });
        // start decoding
        let mut a_or_c = vec![];
        let mut d_or_g = vec![];

        for (k, v) in count {
            if v == 9 {
                cypher_map.insert('f', k);
            } else if v == 8 {
                a_or_c.push(k);
            } else if v == 7 {
                d_or_g.push(k);
            } else if v == 6 {
                cypher_map.insert('b', k);
            } else if v == 4 {
                cypher_map.insert('e', k);
            }
        }

        for k in a_or_c {
            if seq_7.contains(&k) && seq_8.contains(&k) && !seq_1.contains(&k) {
                cypher_map.insert('a', k);
            } else {
                cypher_map.insert('c', k);
            }
        }

        for k in d_or_g {
            if seq_8.contains(&k)
                && seq_4.contains(&k)
                && !seq_1.contains(&k)
                && !seq_7.contains(&k)
            {
                cypher_map.insert('d', k);
            } else {
                cypher_map.insert('g', k);
            }
        }

        // make translation table
        let mut translation = HashMap::new();
        translation.insert(get_translation("abcefg", &cypher_map), "0".to_owned());
        translation.insert(get_translation("cf", &cypher_map), "1".to_owned());
        translation.insert(get_translation("acdeg", &cypher_map), "2".to_owned());
        translation.insert(get_translation("acdfg", &cypher_map), "3".to_owned());
        translation.insert(get_translation("bcdf", &cypher_map), "4".to_owned());
        translation.insert(get_translation("abdfg", &cypher_map), "5".to_owned());
        translation.insert(get_translation("abdefg", &cypher_map), "6".to_owned());
        translation.insert(get_translation("acf", &cypher_map), "7".to_owned());
        translation.insert(get_translation("acbdefg", &cypher_map), "8".to_owned());
        translation.insert(get_translation("abcdfg", &cypher_map), "9".to_owned());

        answer2 += output
            .iter()
            .map(|x| decode_output(&translation, x))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }
    println!("{}", answer2);
}
