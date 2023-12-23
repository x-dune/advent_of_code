use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn solve(grid: &[Vec<char>], part2: bool) -> usize {
    let start = (0, 1);
    let y_max = grid.len() as i32;
    let x_max = grid[0].len() as i32;
    let target = (y_max - 1, x_max - 2);

    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut q = vec![(HashSet::from([(0, 0)]), start)];

    let mut costs = vec![];

    while let Some((path, pos)) = q.pop() {
        let current_path = dist.get(&pos).unwrap_or(&0);
        if path.len() < *current_path {
            continue;
        }

        if pos == target {
            costs.push(path.len() - 1);
            if part2 && costs.len() == 100_000 {
                // stop at 100_000 & hope for the best
                break;
            }
        }

        for (dy, dx, forbidden) in [(-1, 0, 'v'), (1, 0, '^'), (0, -1, '>'), (0, 1, '<')] {
            let next = (pos.0 + dy, pos.1 + dx);
            let entry = dist.entry(next).or_insert(0);

            if next.0 >= 0
                && next.0 < y_max
                && next.1 >= 0
                && next.1 < x_max
                && (part2 || grid[next.0 as usize][next.1 as usize] != forbidden)
                && grid[next.0 as usize][next.1 as usize] != '#'
                && !path.contains(&next)
            {
                let next_cost = path.len() + 1;

                if next_cost > *entry {
                    let mut next_path = path.clone();
                    next_path.insert(next);
                    q.push((next_path, next));
                }
            };
        }
    }

    *costs.iter().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = solve(&input, false);
    let answer2 = solve(&input, true);

    println!("{}\n{}", answer1, answer2);
}
