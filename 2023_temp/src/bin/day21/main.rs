use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn get_start(grid: &[Vec<char>]) -> (i32, i32) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (y as i32, x as i32);
            }
        }
    }
    unreachable!();
}

fn solve(grid: &[Vec<char>], steps: u32) -> usize {
    let start = get_start(grid);

    let mut q = vec![start];
    for _ in 0..steps {
        let mut next_q = HashSet::new();
        while let Some((y, x)) = q.pop() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (y + dy, x + dx);

                if next.0 >= 0
                    && next.0 < grid.len() as i32
                    && next.1 >= 0
                    && next.1 < grid[0].len() as i32
                    && grid[next.0 as usize][next.1 as usize] != '#'
                {
                    next_q.insert(next);
                }
            }
        }
        q = next_q.into_iter().collect::<Vec<_>>();
    }
    q.len()
}

fn solve2(grid: &[Vec<char>], steps: i64) -> i64 {
    // don't undestand how this quadratic solution works
    // https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/keaiiq7/?context=3
    let start = get_start(grid);

    let mut q = vec![start];

    let x = steps % grid.len() as i64;
    let step_checks = [x, x + grid.len() as i64, x + (grid.len() as i64 * 2)];
    let mut xs = vec![];
    let mut i = 0;
    loop {
        let mut next_q = HashSet::new();
        while let Some((y, x)) = q.pop() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = (y + dy, x + dx);
                let mut next_bounded = next;
                let y_max = grid.len() as i32;
                let x_max = grid[0].len() as i32;

                if next.0 < 0 {
                    // println!("{} {} {}", next.0, y_max, next.0 % y_max);
                    next_bounded.0 = (y_max + (next.0 % y_max)) % y_max;
                } else if next.0 >= y_max {
                    next_bounded.0 %= y_max;
                }

                if next.1 < 0 {
                    next_bounded.1 = (x_max + (next.1 % x_max)) % x_max;
                } else if next.1 >= x_max {
                    next_bounded.1 %= x_max;
                }

                if grid[next_bounded.0 as usize][next_bounded.1 as usize] != '#' {
                    next_q.insert(next);
                }
            }
        }
        q = next_q.into_iter().collect::<Vec<_>>();

        i += 1;
        if step_checks.contains(&i) {
            xs.push(q.len() as i64);
            if xs.len() == step_checks.len() {
                break;
            }
        }
    }

    let y1 = xs[1] - xs[0];
    let y2 = xs[2] - xs[1];
    let n = steps / grid.len() as i64;

    xs[0] + y1 * n + ((n * (n - 1)) / 2) * (y2 - y1)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = solve(&input, 64);
    let answer2 = solve2(&input, 26501365);

    println!("{}\n{}", answer1, answer2);
}
