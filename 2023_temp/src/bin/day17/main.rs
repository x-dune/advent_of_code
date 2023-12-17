use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    io::{self, BufRead},
};

const UDLR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DURL: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn solve(grid: &[Vec<u32>], part2: bool) -> i32 {
    let y_max = grid.len() as i32;
    let x_max = grid[0].len() as i32;

    let mut q = BinaryHeap::from_iter([Reverse((0, 0, 0, -1, 0))]);
    let mut dist: BTreeMap<(i32, i32, i32, i32), i32> = BTreeMap::new();

    // cost, (y, x), (dir_index, travelled_in_dir)
    while let Some(Reverse((c, y, x, d, t))) = q.pop() {
        if y == y_max - 1 && x == x_max - 1 {
            return c;
        }
        let key = (y, x, d, t);
        if dist.contains_key(&key) {
            continue;
        }
        dist.insert(key, c);
        for (i, (dy, dx)) in UDLR.iter().enumerate() {
            let next_y = y + dy;
            let next_x = x + dx;
            let next_d = i as i32;
            let next_t = if d == next_d { t + 1 } else { 1 };

            let turning_valid = if part2 {
                // max 10 or not turning or have been in this dir for 4 turns or first turn
                next_t <= 10 && (next_d == d || t >= 4 || t == 0)
            } else {
                next_t <= 3
            };

            if next_y >= 0
                && next_y < y_max
                && next_x >= 0
                && next_x < x_max
                // prevent reversing
                && (d < 0 || DURL[d as usize] != (*dy, *dx))
                && turning_valid
            {
                let next_cost = grid[next_y as usize][next_x as usize];
                q.push(Reverse((
                    c + next_cost as i32,
                    next_y,
                    next_x,
                    next_d,
                    next_t,
                )));
            }
        }
    }
    unreachable!();
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer1 = solve(&input, false);
    let answer2 = solve(&input, true);
    println!("{}\n{}", answer1, answer2);
}
