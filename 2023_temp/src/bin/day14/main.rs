use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

fn get_north_load(grid: &Grid) -> usize {
    let mut load = 0;
    for (y, line) in grid.iter().enumerate() {
        for (_, c) in line.iter().enumerate() {
            if *c == 'O' {
                load += grid.len() - y;
            }
        }
    }
    load
}

fn spin_north(grid: &Grid) -> Grid {
    let mut next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    // north
    for x in 0..grid.len() {
        let mut current_y = 0;
        (0..grid[0].len()).for_each(|y| {
            let current = grid[y][x];
            if current == '#' {
                next_grid[y][x] = '#';
                current_y = y + 1;
            } else if current == 'O' {
                next_grid[current_y][x] = 'O';
                current_y += 1;
            }
        });
    }

    next_grid
}

fn spin_south(grid: &Grid) -> Grid {
    let mut next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    for x in 0..grid.len() {
        let mut current_y = grid[0].len() - 1;
        (0..grid[0].len()).for_each(|y| {
            let y = grid[0].len() - 1 - y;
            let current = grid[y][x];
            if current == '#' {
                next_grid[y][x] = '#';
                current_y = y.saturating_sub(1);
            } else if current == 'O' {
                next_grid[current_y][x] = 'O';
                current_y = current_y.saturating_sub(1);
            }
        });
    }

    next_grid
}

fn spin_west(grid: &Grid) -> Grid {
    let mut next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    // north
    for y in 0..grid.len() {
        let mut current_x = 0;
        (0..grid[0].len()).for_each(|x| {
            let current = grid[y][x];
            if current == '#' {
                next_grid[y][x] = '#';
                current_x = x + 1;
            } else if current == 'O' {
                next_grid[y][current_x] = 'O';
                current_x += 1;
            }
        });
    }

    next_grid
}

fn spin_east(grid: &Grid) -> Grid {
    let mut next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    // north
    for y in 0..grid.len() {
        let mut current_x = grid[0].len() - 1;
        (0..grid[0].len()).for_each(|x| {
            let x = grid[0].len() - 1 - x;
            let current = grid[y][x];
            if current == '#' {
                next_grid[y][x] = '#';
                current_x = x.saturating_sub(1);
            } else if current == 'O' {
                next_grid[y][current_x] = 'O';
                current_x = current_x.saturating_sub(1);
            }
        });
    }

    next_grid
}

fn spin_cycle(grid: &Grid) -> Grid {
    spin_east(&spin_south(&spin_west(&spin_north(grid))))
}

fn solve2(grid: &Grid) -> usize {
    let mut north_loads = vec![];
    let mut i = 0;
    let mut current = grid.clone();
    while i < 1000 {
        let next: Vec<Vec<char>> = spin_cycle(&current);
        if current == next {
            return i;
        } else {
            north_loads.push(get_north_load(&next));
            current = next;
            i += 1;
        }
    }

    // detect cycles
    let last = north_loads.last().unwrap();
    let cycle_length = &north_loads[0..north_loads.len() - 1]
        .iter()
        .rev()
        .position(|x| x == last)
        .unwrap()
        + 1;

    let rem = (1000000000 - 1) % cycle_length;
    let north_load = &north_loads[0..north_loads.len()]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, x)| {
            if i % cycle_length == rem {
                Some(x)
            } else {
                None
            }
        })
        .unwrap();

    **north_load
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = get_north_load(&spin_north(&input));
    let answer2 = solve2(&input);

    println!("{}\n{}", answer1, answer2);
}
