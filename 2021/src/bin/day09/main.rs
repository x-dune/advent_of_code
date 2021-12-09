use std::collections::{BTreeMap, HashSet, VecDeque};

use aoc2021::util;

fn parse_input() -> BTreeMap<(i32, i32), u32> {
    let lines = util::input_as_lines(include_str!("input.txt"));

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

fn basin_size(input: &BTreeMap<(i32, i32), u32>, lowest_point: (i32, i32)) -> usize {
    // Traverse every basin node, outwards from the lowest_point
    let mut visited = HashSet::new();
    let mut remaining = VecDeque::from([lowest_point]);

    while remaining.len() > 0 {
        let current @ (x, y) = remaining.pop_front().unwrap();

        if !visited.insert(current) {
            continue;
        }

        let neighbours = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        let new_remaining = neighbours.iter().filter_map(|p| {
            if let Some(&height) = input.get(p) {
                if !visited.contains(p) && height != 9 {
                    return Some(p);
                }
            }

            return None;
        });

        for &r in new_remaining {
            remaining.push_back(r);
        }
    }

    visited.len()
}

fn main() {
    let input = parse_input();

    let lowest_points = input
        .iter()
        .filter(|(&(x, y), &height)| {
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter_map(|p| input.get(p))
                .all(|d_height| height < *d_height)
        })
        .collect::<Vec<_>>();

    let answer1: u32 = lowest_points.iter().map(|(_, height)| *height + 1).sum();

    println!("{}", answer1);

    let mut basin_sizes = lowest_points
        .iter()
        .map(|(&point, _)| basin_size(&input, point))
        .collect::<Vec<_>>();
    basin_sizes.sort();

    let answer2: usize = basin_sizes.iter().rev().take(3).product();

    println!("{}", answer2);
}
