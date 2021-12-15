use std::{
    cmp::{max, Reverse},
    collections::{BTreeMap, BinaryHeap},
};

fn get_risk_level(grid: &BTreeMap<(i32, i32), i32>) -> Option<i32> {
    let start = (0, 0);
    let end = grid.iter().fold((0, 0), |acc, (curr, _)| {
        (max(acc.0, curr.0 as i32), max(acc.1, curr.1 as i32))
    });

    let mut dist = BTreeMap::new();
    let mut q = BinaryHeap::new();
    let mut prev = BTreeMap::new();
    q.push(Reverse((0, start)));
    dist.insert(start, 0);

    while q.len() > 0 {
        let Reverse((cost, pos)) = q.pop().unwrap();
        if pos == end {
            return Some(cost);
        }

        if cost > *dist.get(&pos).unwrap_or(&i32::MAX) {
            continue;
        }

        [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ]
        .iter()
        .filter_map(|n| grid.get(&n).map(|n_cost| (n_cost, n)))
        .for_each(|(n_cost, &n)| {
            let next_cost = cost + n_cost;
            if next_cost < *dist.get(&n).unwrap_or(&i32::MAX) {
                dist.insert(n, next_cost);
                q.push(Reverse((next_cost, n)));
                prev.insert(n, pos);
            }
        })
    }

    return None;
}

fn insert_expanded(
    grid: &mut BTreeMap<(i32, i32), i32>,
    pos: (i32, i32),
    risk: i32,
    original_grid_size: i32,
) {
    for i in 0..5 {
        let left_most_risk = risk + i;
        [
            left_most_risk,
            left_most_risk + 1,
            left_most_risk + 2,
            left_most_risk + 3,
            left_most_risk + 4,
        ]
        .iter()
        .map(|&r| if r > 9 { r % 9 } else { r })
        .enumerate()
        .for_each(|(j, r)| {
            grid.insert(
                (
                    pos.0 + (original_grid_size * j as i32),
                    pos.1 + (original_grid_size * i),
                ),
                r,
            );
        })
    }
}

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .enumerate()
        .fold(BTreeMap::new(), |mut acc, (y, v)| {
            for (x, risk_level) in v.into_iter().enumerate() {
                acc.insert((x as i32, y as i32), risk_level);
            }
            acc
        });

    let original_grid_size = (grid.len() as f64).sqrt() as i32;

    let grid_expanded = include_str!("input.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .enumerate()
        .fold(BTreeMap::new(), |mut acc, (y, v)| {
            for (x, risk_level) in v.into_iter().enumerate() {
                insert_expanded(
                    &mut acc,
                    (x as i32, y as i32),
                    risk_level,
                    original_grid_size,
                );
            }
            acc
        });

    println!("{:?}", get_risk_level(&grid));
    println!("{:?}", get_risk_level(&grid_expanded));
}
