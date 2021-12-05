use std::cmp::{max, min};

use aoc2021::util;

type Coord = (usize, usize);
type CoordPair = (Coord, Coord);
type Grid = Vec<Vec<usize>>;

fn parse_input() -> Vec<CoordPair> {
    let input = util::input_as_lines(include_str!("input.txt"));
    input
        .iter()
        .map(|x| {
            let splitted = x.split(" -> ").collect::<Vec<&str>>();
            let points1 = splitted[0]
                .split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let points2 = splitted[1]
                .split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            ((points1[0], points1[1]), (points2[0], points2[1]))
        })
        .collect()
}

//Largest coord points
fn generate_empty_grid(input: &Vec<CoordPair>) -> Grid {
    let mut height = 0usize;
    let mut width = 0usize;

    for ((x1, y1), (x2, y2)) in input {
        let max_y = *max(y1, y2) + 1;
        if max_y > height {
            height = max_y;
        }
        let max_x = *max(x1, x2) + 1;
        if max_x > width {
            width = max_x;
        }
    }
    let row = vec![0usize; width as usize];
    let grid = vec![row; height as usize];
    return grid;
}

fn mark_grid(grid: &mut Grid, ((x1, y1), (x2, y2)): &CoordPair) {
    if x1 == x2 {
        let larger_y = *max(y1, y2);
        let smaller_y = *min(y1, y2);

        for y in smaller_y..=larger_y {
            grid[y][*x1] += 1;
        }
    } else if y1 == y2 {
        let larger_x = *max(x1, x2);
        let smaller_x = *min(x1, x2);

        for x in smaller_x..=larger_x {
            grid[*y1][x] += 1;
        }
    }
}

fn is_diagonal(((x1, y1), (x2, y2)): &CoordPair) -> bool {
    return (*x1 as i32 - *y1 as i32).abs() == (*x2 as i32 - *y2 as i32).abs()
        || (*x1 as i32 - *x2 as i32).abs() == (*y1 as i32 - *y2 as i32).abs();
}
fn is_slope_positive(((x1, y1), (x2, y2)): &CoordPair) -> bool {
    return (*y2 as i32 - *y1 as i32) / (*x2 as i32 - *x1 as i32) > 0;
}

fn arrange_coord_based_on_smaller_x(coord_pair @ ((x1, y1), (x2, y2)): &CoordPair) -> CoordPair {
    if x1 > x2 {
        return ((*x2, *y2), (*x1, *y1));
    } else {
        return *coord_pair;
    }
}

fn mark_grid_with_diagonal(grid: &mut Grid, coord_pair @ ((x1, y1), (x2, y2)): &CoordPair) {
    if is_diagonal(coord_pair) {
        if is_slope_positive(coord_pair) {
            let diff = (*y2 as i32 - *y1 as i32).abs() as usize;
            let min_y = min(y1, y2);
            let min_x = min(x1, x2);

            for a in 0..=diff {
                grid[min_y + a][min_x + a] += 1;
            }
        } else {
            let ((xx1, yy1), (xx2, _)) = arrange_coord_based_on_smaller_x(coord_pair);
            let diff = xx2 - xx1;

            for a in 0..=diff {
                grid[yy1 - a][xx1 + a] += 1;
            }
        }
    } else if x1 == x2 {
        let larger_y = *max(y1, y2);
        let smaller_y = *min(y1, y2);

        for y in smaller_y..=larger_y {
            grid[y][*x1] += 1;
        }
    } else if y1 == y2 {
        let larger_x = *max(x1, x2);
        let smaller_x = *min(x1, x2);

        for x in smaller_x..=larger_x {
            grid[*y1][x] += 1;
        }
    }
}

fn count_at_least_2(grid: &Grid) -> usize {
    let mut sum = 0usize;
    for line in grid {
        for cell in line {
            if cell >= &2 {
                sum += 1;
            }
        }
    }
    sum
}

fn solution1() -> usize {
    let input = parse_input();
    let mut grid = generate_empty_grid(&input);

    for coord_pair in input {
        mark_grid(&mut grid, &coord_pair);
    }

    return count_at_least_2(&grid);
}

fn solution2() -> usize {
    let input = parse_input();
    let mut grid = generate_empty_grid(&input);

    for coord_pair in input {
        mark_grid_with_diagonal(&mut grid, &coord_pair);
    }

    return count_at_least_2(&grid);
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
