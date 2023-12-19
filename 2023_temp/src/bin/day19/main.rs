use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Rule {
    // category, op, num, result
    Condition(char, char, i64, String),
    // result
    NoCondition(String),
}
// xmas
type Part = (i64, i64, i64, i64);

fn parse_input(input: &[String]) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let split = input.iter().position(|x| x.is_empty()).unwrap();
    let map = input
        .iter()
        .take(split)
        .map(|line| {
            let (name, rules) = line[0..line.len() - 1].split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|x| {
                    if x.contains(':') {
                        let chars = x.chars().collect::<Vec<_>>();
                        let category = chars[0];
                        let op = chars[1];
                        let (num_raw, result) = x[2..].split_once(':').unwrap();
                        let num = num_raw.parse::<i64>().unwrap();
                        Rule::Condition(category, op, num, result.to_owned())
                    } else {
                        Rule::NoCondition(x.to_owned())
                    }
                })
                .collect::<Vec<_>>();
            (name.to_owned(), rules)
        })
        .collect();

    let parts = input[split + 1..]
        .iter()
        .map(|line| {
            let ratings = line[1..line.len() - 1]
                .split(',')
                .map(|x| x[2..].parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (ratings[0], ratings[1], ratings[2], ratings[3])
        })
        .collect();

    (map, parts)
}

fn solve1(map: &HashMap<String, Vec<Rule>>, parts: &[Part]) -> i64 {
    let mut total = 0;
    for (x, m, a, s) in parts {
        let mut id = "in";
        while id != "A" && id != "R" {
            let workflow = map.get(id).unwrap();
            for rule in workflow {
                match rule {
                    Rule::Condition(category, op, num, result) => {
                        let rating = match category {
                            'x' => x,
                            'm' => m,
                            'a' => a,
                            _ => s,
                        };

                        if *op == '<' {
                            if rating < num {
                                id = result;
                                break;
                            }
                        } else if rating > num {
                            id = result;
                            break;
                        }
                    }
                    Rule::NoCondition(result) => {
                        id = result;
                        break;
                    }
                };
            }
        }
        if id == "A" {
            total += x + m + a + s;
        }
    }
    total
}

fn solve2(map: &HashMap<String, Vec<Rule>>) -> i64 {
    let mut q = vec![("in", 0, [(1, 4000), (1, 4000), (1, 4000), (1, 4000)])];
    let mut total = 0i64;
    while let Some((id, rule_index, ranges)) = q.pop() {
        if id == "A" {
            total += ranges
                .iter()
                .fold(1, |acc, curr| acc * (curr.1 - curr.0 + 1));
        } else if id == "R" {
            continue;
        } else {
            let workflow = map.get(id).unwrap();
            let rule = &workflow[rule_index];
            match rule {
                Rule::Condition(category, op, num, next_id) => {
                    let range_index = match category {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        _ => 3,
                    };

                    if *op == '<' {
                        let mut true_ranges = ranges;
                        true_ranges[range_index].1 = cmp::min(true_ranges[range_index].1, *num - 1);
                        let mut false_ranges = ranges;
                        false_ranges[range_index].0 = cmp::max(false_ranges[range_index].0, *num);

                        q.push((&next_id, 0, true_ranges));
                        q.push((id, rule_index + 1, false_ranges));
                    } else {
                        let mut true_ranges = ranges;
                        true_ranges[range_index].0 = cmp::max(true_ranges[range_index].0, num + 1);
                        let mut false_ranges = ranges;
                        false_ranges[range_index].1 = cmp::min(false_ranges[range_index].1, *num);

                        q.push((&next_id, 0, true_ranges));
                        q.push((id, rule_index + 1, false_ranges));
                    }
                }
                Rule::NoCondition(next_id) => {
                    q.push((&next_id, 0, ranges));
                }
            };
        }
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let (map, parts) = parse_input(&input);
    let answer1 = solve1(&map, &parts);
    let answer2 = solve2(&map);

    println!("{}\n{}", answer1, answer2);
}
