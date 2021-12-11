use std::collections::{BTreeMap, HashSet, VecDeque};

use aoc2021::util;

type GridMap = BTreeMap<(i32, i32), u32>;

fn step(grid_map: GridMap) -> (GridMap, usize) {
    let mut q = VecDeque::new();
    let mut next_grid_map = grid_map
        .iter()
        .map(|(coord, energy)| {
            let next_energy = *energy + 1;
            if next_energy > 9 {
                q.push_back(*coord);
            }
            (*coord, *energy + 1)
        })
        .collect::<GridMap>();
    let mut flashed = HashSet::new();

    while q.len() > 0 {
        let current @ (x, y) = q.pop_front().unwrap();
        if !flashed.insert(current) {
            continue;
        }

        *next_grid_map.get_mut(&current).unwrap() = 0;

        let neighbours = [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
        ];

        for d in neighbours {
            if flashed.contains(&d) {
                continue;
            }

            if let Some(energy) = next_grid_map.get_mut(&d) {
                *energy += 1;
                if *energy > 9 {
                    q.push_back(d)
                }
            }
        }
    }

    (next_grid_map, flashed.len())
}

fn main() {
    let mut grid_map = util::input_as_lines(include_str!("input.txt"))
        .iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut acc, (y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                acc.insert((x as i32, y as i32), c.to_digit(10).unwrap());
            });
            acc
        });

    let mut total_flashes = 0;
    let mut has_all_flashed = false;
    for i in 1.. {
        let (next_grid_map, flashes) = step(grid_map);
        total_flashes += flashes;
        grid_map = next_grid_map;

        if flashes == grid_map.len() {
            has_all_flashed = true;
        }

        if i == 100 {
            println!("part 1: {}", total_flashes);
        }

        if has_all_flashed {
            println!("part 2: {}", i);
        }

        if i >= 100 && has_all_flashed {
            break;
        }
    }
}
