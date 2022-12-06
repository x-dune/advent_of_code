use std::collections::HashMap;

const HALLWAY_LEN: usize = 11;
const DEST_LOCATION: [usize; 4] = [2, 4, 6, 8];
type Grid = Vec<Vec<char>>;

fn parse_input(part2: bool) -> Grid {
    let mut lines = include_str!("input.txt")
        .lines()
        .map(|s| {
            let padded = format!("{:#<13}", s).replace(' ', "#");
            return padded[1..padded.len() - 1].chars().collect();
        })
        .collect::<Vec<_>>();
    lines.remove(0);
    lines.pop();

    if part2 {
        let temp = lines.pop().unwrap();
        lines.push(format!("{:#^11}", "#D#C#B#A#").chars().collect());
        lines.push(format!("{:#^11}", "#D#B#A#C#").chars().collect());
        lines.push(temp);
    }
    lines
}

fn is_front_in_home(x: usize, y: usize, grid: &Grid) -> bool {
    assert!(DEST_LOCATION.contains(&x));
    y == 1 || y >= 1 && (1..y).all(|dy| grid[dy][x] == '.')
}

fn is_path_clear(x: usize, target_x: usize, grid: &Grid) -> bool {
    assert_ne!(x, target_x);
    if x < target_x {
        return grid[0][x + 1..target_x].iter().all(|&tile| tile == '.');
    } else {
        return grid[0][target_x + 1..x].iter().all(|&tile| tile == '.');
    }
}

fn is_done(grid: &Grid) -> bool {
    (1..grid.len())
        .all(|y| grid[y][2] == 'A' && grid[y][4] == 'B' && grid[y][6] == 'C' && grid[y][8] == 'D')
}

fn is_amp_in_place(x: usize, y: usize, grid: &Grid) -> bool {
    let amp = grid[y][x];
    let dest_map = HashMap::from([('A', 2), ('B', 4), ('C', 6), ('D', 8)]);
    let dest_x = dest_map[&amp];

    x == dest_x && (y + 1..grid.len()).all(|dy| grid[dy][x] == amp)
}

fn hallway_to_home(amp: char, x: usize, grid: &Grid) -> Option<(usize, usize)> {
    let dest_map = HashMap::from([('A', 2), ('B', 4), ('C', 6), ('D', 8)]);
    let dest_x = dest_map[&amp];
    let is_home_open = (1..grid.len()).all(|dy| grid[dy][dest_x] == '.' || grid[dy][dest_x] == amp);
    if !is_home_open {
        return None;
    }
    if !is_path_clear(x, dest_x, grid) {
        return None;
    }

    let mut dest_y = 1;
    for (dy, _) in grid.iter().enumerate().skip(2) {
        if grid[dy][dest_x] == '.' {
            dest_y = dy;
        }
    }

    Some((dest_x, dest_y))
}

fn home_to_hallway_moves(x: usize, y: usize, grid: &Grid) -> Vec<(usize, usize)> {
    if !is_front_in_home(x, y, grid) {
        return vec![];
    }
    let mut moves = vec![];
    for dx in 0..HALLWAY_LEN {
        if !DEST_LOCATION.contains(&dx) && grid[0][dx] == '.' && is_path_clear(x, dx, grid) {
            moves.push((dx, 0));
        }
    }
    moves
}

fn solve(grid: Grid) -> i32 {
    let cost_map = HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
    let mut states = vec![(grid.clone(), 0)];
    let mut best = HashMap::from([(grid, 0)]);
    let mut done_costs = vec![];

    while let Some((grid, cost)) = states.pop() {
        let mut next_moves = vec![];

        for (x, &tile) in grid[0].iter().enumerate() {
            if tile == '.' {
                continue;
            }

            if let Some((dx, dy)) = hallway_to_home(tile, x, &grid) {
                next_moves.push((x, 0, dx, dy));
            };
        }

        for x in DEST_LOCATION {
            for y in 1..grid.len() {
                let tile = grid[y][x];

                if tile == '.' || is_amp_in_place(x, y, &grid) {
                    continue;
                }
                let possible_moves = home_to_hallway_moves(x, y, &grid);

                possible_moves
                    .iter()
                    .for_each(|&(dx, dy)| next_moves.push((x, y, dx, dy)));
            }
        }

        next_moves.iter().for_each(|&(x, y, dx, dy)| {
            let mut next_grid = grid.clone();
            let tile = next_grid[y][x];
            next_grid[y][x] = '.';
            next_grid[dy][dx] = tile;
            let move_cost =
                ((dx as i32 - x as i32).abs() + (dy as i32 - y as i32).abs()) * cost_map[&tile];
            let next_cost = cost + move_cost;

            let current_cost = best.get(&next_grid);
            if current_cost.is_none() || next_cost < *current_cost.unwrap() {
                best.insert(next_grid.clone(), next_cost);

                if is_done(&next_grid) {
                    done_costs.push(next_cost);
                } else {
                    states.push((next_grid, next_cost));
                }
            }
        });
    }
    done_costs.sort_unstable();
    done_costs[0]
}

fn main() {
    println!("{}", solve(parse_input(false)));
    println!("{}", solve(parse_input(true)));
}
