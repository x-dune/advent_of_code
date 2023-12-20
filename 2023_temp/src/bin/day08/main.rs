use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

use advent_of_code_2023_temp::util;

fn parse_input(map_lines: &[String]) -> (BTreeMap<String, (String, String)>, Vec<String>) {
    let mut ghost_starts: Vec<String> = vec![];

    let map = map_lines
        .iter()
        .map(|x| {
            let (node, left_right) = x.split_once(" = ").unwrap();

            let (left, right) = left_right[1..left_right.len() - 1]
                .split_once(", ")
                .unwrap();

            if node.ends_with('A') {
                ghost_starts.push(node.to_owned());
            }

            (node.to_owned(), (left.to_owned(), right.to_owned()))
        })
        .collect::<BTreeMap<_, _>>();

    (map, ghost_starts)
}

fn navigate(
    start: &str,
    map: &BTreeMap<String, (String, String)>,
    inst: &Vec<char>,
    predicate: fn(&str) -> bool,
) -> i64 {
    let mut current = start;
    let mut steps: i64 = 0;
    while predicate(current) {
        let i = steps as usize % inst.len();
        let direction = inst[i];
        if direction == 'L' {
            current = &map.get(current).unwrap().0;
        } else {
            current = &map.get(current).unwrap().1;
        }
        steps += 1;
    }
    steps
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let inst = input[0].chars().collect::<Vec<_>>();
    let (map, ghost_starts) = parse_input(&input[2..]);

    let answer1 = navigate("AAA", &map, &inst, |x| x != "ZZZ");
    let answer2 = util::lcm(
        ghost_starts
            .iter()
            .map(|x| navigate(x, &map, &inst, |x| !x.ends_with('Z')))
            .collect(),
    );

    println!("{answer1}\n{:?}", answer2);
}
