use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

fn expand_universe(grid: &Grid, expansion_factor: usize) -> Vec<Coord> {
    let expansion_factor = if expansion_factor == 1 {
        expansion_factor
    } else {
        expansion_factor - 1
    };
    let mut empty_row_index = vec![];
    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|e| *e == '.') {
            empty_row_index.push(y)
        }
    }

    let mut empty_column_index = vec![];
    for x in 0..grid[0].len() {
        if grid.iter().map(|e| e[x]).all(|e| e == '.') {
            empty_column_index.push(x)
        }
    }

    let mut initial_galaxies = vec![];
    let mut galaxies = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                initial_galaxies.push((y, x));
                let y_expansion =
                    empty_row_index.iter().filter(|e| y > **e).count() * expansion_factor;
                let x_expansion =
                    empty_column_index.iter().filter(|e| x > **e).count() * expansion_factor;
                galaxies.push((y + y_expansion, x + x_expansion))
            }
        }
    }

    galaxies
}

fn get_sum_distances(coords: Vec<Coord>) -> i64 {
    let mut sum = 0;
    let mut seen_pair = HashSet::new();
    let coords2 = coords.clone();
    for (y1, x1) in coords {
        for (y2, x2) in &coords2 {
            if y1 == *y2 && x1 == *x2
                || seen_pair.contains(&(y1, x1, *y2, *x2))
                || seen_pair.contains(&(*y2, *x2, y1, x1))
            {
                continue;
            } else {
                sum += (y1 as i64 - *y2 as i64).abs() + (x1 as i64 - *x2 as i64).abs();
                seen_pair.insert((y1, x1, *y2, *x2));
            }
        }
    }
    // can just divide by 2 instead of keeping track of pairs
    sum
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let galaxy1 = expand_universe(&input, 1);
    let galaxy2 = expand_universe(&input, 1_000_000);
    let answer1 = get_sum_distances(galaxy1);
    let answer2 = get_sum_distances(galaxy2);

    println!("{}\n{}", answer1, answer2);
}
