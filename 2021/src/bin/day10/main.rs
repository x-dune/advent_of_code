use aoc2021::util;

const CHUNK_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
const CHUNK_OPENERS: [char; 4] = ['(', '[', '{', '<'];

enum SyntaxErrorType {
    IllegalCharacter(char),
    IncompleteLine(Vec<char>),
}

fn syntax_error_type(line: &str) -> SyntaxErrorType {
    let mut stack = vec![];

    for c in line.chars() {
        if stack.is_empty() || CHUNK_OPENERS.contains(&c) {
            stack.push(c);
        } else {
            let opener = stack.pop().unwrap();

            if !CHUNK_PAIRS.contains(&(opener, c)) {
                return SyntaxErrorType::IllegalCharacter(c);
            }
        }
    }

    let reversed = stack.into_iter().rev().collect();
    return SyntaxErrorType::IncompleteLine(reversed);
}

fn main() {
    let input = util::input_as_lines(include_str!("input.txt"));

    let (illegal_chars, incommplete_lines) =
        input
            .iter()
            .map(|line| syntax_error_type(line))
            .fold((vec![], vec![]), |mut acc, curr| {
                match curr {
                    SyntaxErrorType::IllegalCharacter(c) => acc.0.push(c),
                    SyntaxErrorType::IncompleteLine(chars) => acc.1.push(chars),
                }
                acc
            });

    let answer1: i32 = illegal_chars
        .iter()
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum();
    println!("{}", answer1);

    let mut part2_scores = incommplete_lines
        .iter()
        .map(|chars| {
            chars.iter().fold(0i64, |acc, curr| {
                acc * 5
                    + match curr {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(),
                    }
            })
        })
        .collect::<Vec<_>>();
    part2_scores.sort();
    let answer2 = part2_scores[part2_scores.len() / 2];

    println!("{}", answer2);
}
