use std::{cmp::max, collections::HashSet};

fn fold_on_y(paper: &HashSet<(i32, i32)>, n: i32) -> HashSet<(i32, i32)> {
    let mut next_paper = HashSet::new();

    for &(x, y) in paper.iter() {
        let diff = y - &n;

        if diff > 0 {
            next_paper.insert((x, &n - diff));
        } else {
            next_paper.insert((x, y));
        }
    }
    next_paper
}

fn fold_on_x(paper: &HashSet<(i32, i32)>, n: i32) -> HashSet<(i32, i32)> {
    let mut next_paper = HashSet::new();

    for &(x, y) in paper.iter() {
        let diff = x - &n;

        if diff > 0 {
            next_paper.insert((&n - diff, y));
        } else {
            next_paper.insert((x, y));
        }
    }
    next_paper
}

fn print_paper(paper: &HashSet<(i32, i32)>) {
    let (max_x, max_y) = paper
        .iter()
        .fold((0, 0), |acc, curr| (max(acc.0, curr.0), max(acc.1, curr.1)));

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        print!("\n")
    }
}

fn main() {
    let (dots_raw, instructions_raw) = include_str!("input.txt").split_once("\n\n").unwrap();

    let dots = dots_raw
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<(i32, i32)>>();

    let instructions = instructions_raw
        .lines()
        .map(|s| {
            let (dir, n) = &s[11..].split_once("=").unwrap();

            (*dir, n.parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    let mut paper = dots.iter().fold(HashSet::new(), |mut acc, curr| {
        acc.insert(*curr);
        acc
    });

    for (i, &(dir, n)) in instructions.iter().enumerate() {
        match dir {
            "x" => paper = fold_on_x(&paper, n),
            "y" => paper = fold_on_y(&paper, n),
            _ => panic!(),
        }

        if i == 0 {
            println!("part 1 {}", &paper.len())
        }
    }

    println!("part 2, squint to read answer");
    print_paper(&paper);
}
