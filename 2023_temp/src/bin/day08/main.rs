use std::{
    collections::{BTreeMap, HashMap},
    io::{self, BufRead},
};

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

fn get_prime_factors(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut result = vec![];

    while n % 2 == 0 {
        result.push(2);
        n /= 2;
    }

    let mut i = 3;
    while i <= (n as f64).sqrt().floor() as i64 {
        i += 2;

        while n % i == 0 {
            result.push(i);
            n /= i;
        }
    }

    if n > 2 {
        result.push(n);
    }

    result
}

fn get_lcm(ns: Vec<i64>) -> i64 {
    let prime_factors_of_ns = ns.into_iter().map(get_prime_factors).collect::<Vec<_>>();

    let mut max_count_of_factor: HashMap<i64, i64> = HashMap::new();

    for factors in prime_factors_of_ns {
        let mut count_of_factor: HashMap<i64, i64> = HashMap::new();
        for factor in factors {
            *count_of_factor.entry(factor).or_insert(0) += 1;
        }

        for (factor, count) in count_of_factor.into_iter() {
            let max_count = max_count_of_factor.entry(factor).or_insert(0);

            if count > *max_count {
                *max_count = count;
            }
        }
    }

    max_count_of_factor
        .iter()
        .fold(1, |acc, (factor, count)| acc * factor.pow(*count as u32))
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
    let answer2 = get_lcm(
        ghost_starts
            .iter()
            .map(|x| navigate(x, &map, &inst, |x| !x.ends_with('Z')))
            .collect(),
    );

    println!("{answer1}\n{:?}", answer2);
}
