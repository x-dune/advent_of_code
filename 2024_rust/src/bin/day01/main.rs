use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn parse_input(s: &[String]) -> (Vec<i64>, Vec<i64>) {
    s.iter()
        .map(|x| {
            let (a, b) = x.split_once(' ').unwrap();
            (
                a.parse::<i64>().ok().unwrap(),
                b.trim().parse::<i64>().ok().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let (list1, list2) = parse_input(&input);
    let (mut sorted_list1, mut sorted_list2) = (list1.clone(), list2.clone());
    sorted_list1.sort();
    sorted_list2.sort();

    let mut answer1 = 0;
    let mut answer2 = 0;

    let mut counts: HashMap<i64, i64> = HashMap::new();

    sorted_list2.iter().for_each(|x| {
        *counts.entry(*x).or_insert(0) += 1;
    });

    for i in 0..sorted_list1.len() {
        answer1 += (sorted_list1[i] - sorted_list2[i]).abs();
        let count = counts.get(&sorted_list1[i]).unwrap_or(&0);
        answer2 += sorted_list1[i] * count;
    }

    println!("{}\n{}", answer1, answer2);
}
