use std::{
    cmp::{max, Reverse},
    collections::{BTreeMap, BinaryHeap},
};

fn dijkstra(grid: &BTreeMap<(i32, i32), i32>) -> i32 {
    let start = (0, 0);
    let end = grid.iter().fold((0, 0), |acc, (cord, _)| {
        (max(acc.0, cord.0), max(acc.1, cord.1))
    });

    let mut dist = BTreeMap::new();
    let mut q = BinaryHeap::new();

    dist.insert(start, 0);
    q.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos @ (x, y)))) = q.pop() {
        if pos == end {
            return cost;
        }

        if cost > *dist.get(&pos).unwrap_or(&i32::MAX) {
            continue;
        }

        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .for_each(|&n_pos| {
                if let Some(d_cost) = grid.get(&n_pos) {
                    let next_cost = d_cost + cost;
                    if next_cost < *dist.get(&n_pos).unwrap_or(&i32::MAX) {
                        dist.insert(n_pos, next_cost);
                        q.push(Reverse((next_cost, n_pos)))
                    }
                }
            });
    }
    unreachable!();
}

fn main() {
    let input = include_str!("input.txt");
    let grid_horizontal_size = input.lines().next().unwrap().len() as i32;
    let grid_vertical_size = input.lines().count() as i32;
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap() as i32))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();

    let answer1 = dijkstra(&grid);

    println!("{}", answer1);

    let expanded_grid = &grid
        .iter()
        .flat_map(|(&pos, &risk)| {
            (0..5).flat_map(move |i| {
                (0..5).map(move |j| {
                    let expanded_risk = risk + i + j;
                    let normalized_risk = if expanded_risk > 9 {
                        expanded_risk % 9
                    } else {
                        expanded_risk
                    };
                    (
                        (
                            pos.0 + i * grid_horizontal_size,
                            pos.1 + j * grid_vertical_size,
                        ),
                        normalized_risk,
                    )
                })
            })
        })
        .collect::<BTreeMap<_, _>>();

    let answer2 = dijkstra(expanded_grid);

    println!("{}", answer2);
}
