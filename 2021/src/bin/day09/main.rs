use std::collections::HashSet;

use aoc2021::util;

fn parse_input() -> Vec<Vec<i32>> {
    let lines = util::input_as_lines(include_str!("input.txt"));

    lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

const HIGH: i32 = 9;

fn get_basin_size(input: &Vec<Vec<i32>>, lowest_point: (usize, usize)) -> i32 {
    let mut q = vec![lowest_point];
    let mut done = vec![];
    while q.len() != 0 {
        let (y, x) = q.pop().unwrap();
        if y != 0usize && input[y - 1][x] != HIGH && !done.contains(&(y - 1, x)) {
            q.push((y - 1, x));
        }
        if y != input.len() - 1 && input[y + 1][x] != HIGH && !done.contains(&(y + 1, x)) {
            q.push((y + 1, x));
        }

        if x != 0usize && input[y][x - 1] != HIGH && !done.contains(&(y, x - 1)) {
            q.push((y, x - 1));
        }

        if x != input[0].len() - 1 && input[y][x + 1] != HIGH && !done.contains(&(y, x + 1)) {
            q.push((y, x + 1));
        }
        done.push((y, x));
    }

    // quick fix
    done.iter()
        .fold(HashSet::new(), |mut acc, curr| {
            acc.insert(curr);
            acc
        })
        .len() as i32
}

fn main() {
    let input = parse_input();

    let mut lowest_point_values = vec![];
    let mut lowest_points = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (y == 0 || cell < &input[y - 1][x])
                && (y == input.len() - 1 || cell < &input[y + 1][x])
                && (x == 0 || cell < &input[y][x - 1])
                && (x == row.len() - 1 || cell < &input[y][x + 1])
            {
                lowest_point_values.push(*cell);
                lowest_points.push((y, x));
            }
        }
    }
    let answer1: i32 = lowest_point_values.iter().sum::<i32>() + lowest_point_values.len() as i32;
    println!("{}", answer1);

    let mut basin_sizes = lowest_points
        .iter()
        .map(|v| get_basin_size(&input, *v))
        .collect::<Vec<i32>>();
    basin_sizes.sort_by(|a, b| b.cmp(a));

    let answer2 = &basin_sizes[..3].iter().product::<i32>();

    println!("{:?}", answer2);
}
