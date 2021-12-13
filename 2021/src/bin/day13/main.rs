use std::{cmp::max, collections::HashSet};

fn fold_paper(paper: &HashSet<(i32, i32)>, (axis, pos): (&str, i32)) -> HashSet<(i32, i32)> {
    paper
        .iter()
        .map(|&(x, y)| match (axis, x > pos, y > pos) {
            ("x", true, _) => (pos * 2 - x, y),
            ("y", _, true) => (x, pos * 2 - y),
            _ => (x, y),
        })
        .collect::<HashSet<_>>()
}

fn format_paper(paper: &HashSet<(i32, i32)>) -> String {
    let mut output = String::from("");
    let (max_x, max_y) = paper
        .iter()
        .fold((0, 0), |acc, curr| (max(acc.0, curr.0), max(acc.1, curr.1)));

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                output.push_str("##")
            } else {
                output.push_str("  ")
            }
        }
        output.push_str("\n")
    }

    output
}

fn main() {
    let (dots, instructions_raw) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut paper = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect();

    let instructions = instructions_raw
        .lines()
        .map(|s| {
            let (axis, n) = &s[11..].split_once("=").unwrap();
            (*axis, n.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    for (i, &instruction) in instructions.iter().enumerate() {
        paper = fold_paper(&paper, instruction);
        if i == 0 {
            println!("part 1 {}", &paper.len())
        }
    }

    println!("part 2\n{}", format_paper(&paper));
}
