use std::collections::HashMap;

use aoc2021::util;

// x1, y1, x2, y2
type CoordPair = (i32, i32, i32, i32);

fn parse_input() -> Vec<CoordPair> {
    let input = util::input_as_lines(include_str!("input.txt"));
    input
        .iter()
        .map(|l| {
            let points = l
                .split(" -> ")
                .flat_map(|s| s.split(','))
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();

            (points[0], points[1], points[2], points[3])
        })
        .collect()
}

fn get_overlapping_amount(coord_pairs: Vec<CoordPair>) -> usize {
    let mut vent_coords = HashMap::new();

    for (x1, y1, x2, y2) in coord_pairs {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let (mut current_x, mut current_y) = (x1, y1);

        while (current_x, current_y) != (x2 + dx, y2 + dy) {
            *vent_coords.entry((current_x, current_y)).or_insert(0) += 1;
            current_x += dx;
            current_y += dy;
        }
    }

    return vent_coords.values().filter(|&&n| n >= 2).count();
}

fn solution1() -> usize {
    let input = parse_input();
    let input_without_diagonals = input
        .into_iter()
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect();

    return get_overlapping_amount(input_without_diagonals);
}

fn solution2() -> usize {
    let input = parse_input();
    return get_overlapping_amount(input);
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
