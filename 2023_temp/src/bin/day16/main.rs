use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

type Beam = ((i32, i32), (i32, i32)); // (y, x), (dy, dx)
const U: (i32, i32) = (-1, 0);
const D: (i32, i32) = (1, 0);
const L: (i32, i32) = (0, -1);
const R: (i32, i32) = (0, 1);

fn in_bounds((y, x): &(i32, i32), (dy, dx): &(i32, i32), grid: &[Vec<char>]) -> Option<(i32, i32)> {
    let next = (y + dy, x + dx);
    if next.0 >= 0 && next.0 < grid.len() as i32 && next.1 >= 0 && next.1 < grid[0].len() as i32 {
        Some(next)
    } else {
        None
    }
}

fn beam_traversal(start: &Beam, grid: &[Vec<char>]) -> usize {
    let mut beams: Vec<Beam> = vec![*start];
    let mut seen = HashSet::new();

    let forward_slash = HashMap::from([(U, R), (D, L), (L, D), (R, U)]);
    let back_slash = HashMap::from([(U, L), (D, R), (L, U), (R, D)]);

    while let Some((pos, dir)) = beams.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }

        let current_cell = grid[pos.0 as usize][pos.1 as usize];
        let mut nexts_option = vec![];
        if current_cell == '|' && (dir == R || dir == L) {
            nexts_option = vec![
                (in_bounds(&pos, &U, grid), U),
                (in_bounds(&pos, &D, grid), D),
            ];
        } else if current_cell == '-' && (dir == U || dir == D) {
            nexts_option = vec![
                (in_bounds(&pos, &L, grid), L),
                (in_bounds(&pos, &R, grid), R),
            ];
        } else if current_cell == '/' {
            if let Some(next) = in_bounds(&pos, &forward_slash[&dir], grid) {
                nexts_option = vec![(Some(next), forward_slash[&dir])];
            }
        } else if current_cell == '\\' {
            if let Some(next) = in_bounds(&pos, &back_slash[&dir], grid) {
                nexts_option = vec![(Some(next), back_slash[&dir])];
            }
        } else if let Some(next) = in_bounds(&pos, &dir, grid) {
            // '.'
            nexts_option = vec![(Some(next), dir)];
        }

        let nexts = nexts_option
            .iter()
            .filter_map(|(x, y)| x.map(|z| (z, *y)))
            .collect::<Vec<_>>();

        beams.extend(nexts.clone());
        seen.insert((pos, dir));
    }
    let seen_pos: HashSet<(i32, i32)> = HashSet::from_iter(seen.iter().map(|(x, _)| *x));
    seen_pos.len()
}

fn get_edge_starts(grid: &[Vec<char>]) -> Vec<Beam> {
    let mut result = vec![];
    for n in 0..grid.len() {
        let n = n as i32;
        result.push(((0, n), D)); // U edge
        result.push(((grid.len() as i32 - 1, n), U)); // D edge
        result.push(((n, 0), R)); // L edge
        result.push(((n, grid[0].len() as i32 - 1), L)); // R edge
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = beam_traversal(&((0, 0), R), &input);
    let edge_starts = get_edge_starts(&input);
    let answer2 = edge_starts
        .iter()
        .map(|x| beam_traversal(x, &input))
        .max()
        .unwrap();

    println!("{}\n{}", answer1, answer2);
}
