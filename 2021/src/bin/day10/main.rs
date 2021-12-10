use aoc2021::util;

fn get_illegal_character(line: &str) -> Option<char> {
    let mut stack = vec![];

    let chunk_chars = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let chunk_openers = chunk_chars.map(|x| x.0);

    for c in line.chars() {
        if stack.is_empty() || chunk_openers.contains(&c) {
            stack.push(c);
        } else {
            let opener = stack.pop().unwrap();

            if !chunk_chars.contains(&(opener, c)) {
                return Some(c);
            }
        }
    }

    return None;
}

fn complete_lines(line: &str) -> String {
    let mut stack = vec![];

    let chunk_chars = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let chunk_openers = chunk_chars.map(|x| x.0);

    for c in line.chars() {
        if stack.is_empty() || chunk_openers.contains(&c) {
            stack.push(c);
        } else {
            let opener = stack.pop().unwrap();

            if !chunk_chars.contains(&(opener, c)) {
                panic!();
            }
        }
    }

    return stack
        .iter()
        .rev()
        .map(|x| match x {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!(),
        })
        .collect();
}

fn score_completion_string(s: &str) -> i64 {
    s.chars().fold(0, |acc, curr| {
        let next = acc * 5;
        let to_add = match curr {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        };
        next + to_add
    })
}

fn main() {
    let input = util::input_as_lines(include_str!("input.txt"));

    let answer1: i32 = input
        .iter()
        .filter_map(|line| get_illegal_character(line))
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum();

    println!("{}", answer1);

    let mut scores = input
        .iter()
        .filter(|line| get_illegal_character(line).is_none())
        .map(|line| complete_lines(line))
        .map(|s| score_completion_string(&s))
        .collect::<Vec<_>>();
    scores.sort();

    let answer2 = scores[scores.len() / 2];

    println!("{:?}", answer2);
}
