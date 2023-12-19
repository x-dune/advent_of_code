use std::{
    cmp,
    io::{self, BufRead},
};

type Grid = Vec<String>;

fn parse_input(input: &[String]) -> Vec<Grid> {
    input
        .join("\n")
        .split("\n\n")
        .map(|x| x.split('\n').map(|y| y.to_owned()).collect())
        .collect()
}

fn smudge_check(left: &[char], right: &[char], smudge: &mut i32) -> bool {
    for i in 0..left.len() {
        if left[i] != right[i] {
            if *smudge == 0 {
                return false;
            } else {
                *smudge -= 1;
            }
        }
    }
    true
}

fn get_reflection(grid: &Grid, smudge: i32) -> i32 {
    // there's len - 1 reflection points, since its between indexes
    // checking for vertical reflection
    let mut i = 1;
    while i < grid[0].len() {
        let checks = cmp::min(grid[0].len() - i, i);
        let mut valid = true;
        let mut smudge = smudge;
        for line in grid {
            let left_side = &line[i - checks..i].chars().rev().collect::<Vec<_>>();
            let right_side = &line[i..i + checks].chars().collect::<Vec<_>>();
            if !smudge_check(left_side, right_side, &mut smudge) {
                valid = false;
                break;
            }
        }
        if valid && smudge == 0 {
            return i as i32;
        }
        i += 1;
    }
    // checking for horizontal reflection
    i = 1;
    while i < grid.len() {
        let checks = cmp::min(grid.len() - i, i);
        let mut valid = true;
        let mut smudge = smudge;
        for j in 0..grid[0].len() {
            let line = (0..grid.len())
                .map(|x| {
                    let chars = &grid[x][j..j + 1].chars().collect::<Vec<_>>();
                    chars[0]
                })
                .collect::<String>();
            let left = &line[i - checks..i].chars().rev().collect::<Vec<_>>();
            let right = &line[i..i + checks].chars().collect::<Vec<_>>();
            if !smudge_check(left, right, &mut smudge) {
                valid = false;
                break;
            }
        }
        if valid && smudge == 0 {
            return i as i32 * 100;
        }
        i += 1;
    }
    // failed to find reflection
    0
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let parsed = parse_input(&input);
    let answer1 = parsed.iter().map(|x| get_reflection(x, 0)).sum::<i32>();
    let answer2 = parsed.iter().map(|x| get_reflection(x, 1)).sum::<i32>();

    println!("{}\n{}", answer1, answer2);
}
