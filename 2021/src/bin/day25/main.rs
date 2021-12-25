fn step(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut did_move = false;
    let mut grid = grid.clone();
    let mut next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == '>' {
                if x + 1 == grid[0].len() {
                    if grid[y][0] == '.' {
                        next_grid[y][0] = '>';
                        did_move = true;
                        continue;
                    }
                } else if grid[y][x + 1] == '.' {
                    next_grid[y][x + 1] = '>';
                    did_move = true;
                    continue;
                }
                next_grid[y][x] = '>'
            } else if col == 'v' {
                next_grid[y][x] = 'v';
            }
        }
    }

    grid = next_grid;
    next_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == '>' {
                next_grid[y][x] = '>'
            } else if col == 'v' {
                if y == grid.len() - 1 {
                    if grid[0][x] == '.' {
                        next_grid[0][x] = 'v';
                        did_move = true;
                        continue;
                    }
                } else if grid[y + 1][x] == '.' {
                    next_grid[y + 1][x] = 'v';
                    did_move = true;
                    continue;
                }
                next_grid[y][x] = 'v';
            }
        }
    }

    (next_grid, did_move)
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|a| a.chars().collect())
        .collect::<Vec<_>>();

    let mut grid = input;
    for i in 1.. {
        let (next_grid, did_move) = step(&grid);
        grid = next_grid;
        if !did_move {
            println!("{}", i);
            break;
        }
    }
}
