use std::collections::{BTreeMap, HashSet, VecDeque};

use aoc2021::util;

type Grid = BTreeMap<(i32, i32), u32>;

fn step(grid: Grid) -> (Grid, usize) {
    let mut next_grid = grid
        .iter()
        .map(|(coord, energy)| (*coord, *energy + 1))
        .collect::<Grid>();
    let mut to_flash = next_grid
        .iter()
        .filter_map(|(coord, energy)| {
            if energy > &&9 {
                return Some(*coord);
            }
            return None;
        })
        .fold(VecDeque::new(), |mut acc, curr| {
            acc.push_back(curr);
            acc
        });
    let mut has_flashed = HashSet::new();

    while to_flash.len() > 0 {
        let current @ (x, y) = to_flash.pop_front().unwrap();
        if !has_flashed.insert(current) {
            continue;
        }

        *next_grid.get_mut(&current).unwrap() = 0;

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
            if has_flashed.contains(&d) {
                continue;
            }

            if let Some(energy) = next_grid.get_mut(&d) {
                *energy += 1;
                if *energy > 9 {
                    to_flash.push_back(d)
                }
            }
        }
    }

    (next_grid, has_flashed.len())
}

fn main() {
    let input = util::input_as_lines(include_str!("input.txt"))
        .iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut acc, (y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                acc.insert((x as i32, y as i32), c.to_digit(10).unwrap());
            });
            acc
        });

    let mut input_clone = input.clone();
    let mut total_flashes = 0;
    let mut has_all_flashed = false;
    for i in 1.. {
        let (next_grid, flashes) = step(input_clone);
        total_flashes += flashes;
        input_clone = next_grid;

        if flashes == 100 {
            has_all_flashed = true;
        }

        if i == 100 {
            println!("{}", total_flashes);
            if has_all_flashed {
                break;
            }
        }

        if has_all_flashed && i > 100 {
            println!("{}", i);
            break;
        }
    }
}
