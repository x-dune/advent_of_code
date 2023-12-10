use std::{
    collections::{BTreeSet, HashMap},
    io::{self, BufRead},
};

const CONNECT_TO_TOP: [char; 3] = ['|', '7', 'F'];
const CONNECT_TO_BOTTOM: [char; 3] = ['|', 'L', 'J'];
const CONNECT_TO_LEFT: [char; 3] = ['-', 'L', 'F'];
const CONNECT_TO_RIGHT: [char; 3] = ['-', 'J', '7'];

fn get_valid_offset(
    coord: (usize, usize),
    offset: (i32, i32),
    y_max: usize,
    x_max: usize,
    seen: &BTreeSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let coord = (coord.0 as i32, coord.1 as i32);
    let offset = (offset.0, offset.1);
    let y_max = y_max as i32;
    let x_max = x_max as i32;

    let next = (coord.0 + offset.0, coord.1 + offset.1);

    if next.0 >= 0 && next.0 <= y_max && next.1 >= 0 && next.1 <= x_max {
        let next = (next.0 as usize, next.1 as usize);
        if !seen.contains(&next) {
            return Some(next);
        }
    }
    None
}

fn traverse(
    start: (usize, usize),

    grid: Vec<Vec<char>>,
) -> (i32, BTreeSet<(usize, usize)>, Vec<Vec<char>>) {
    let mut seen = BTreeSet::from([start]);
    let mut q = vec![(start, 'S')];
    let mut i = -1;

    let y_max = grid.len();
    let x_max = grid[0].len();

    let tile_mapping = HashMap::from([
        (
            'S',
            vec![
                ((-1, 0), CONNECT_TO_TOP),
                ((1, 0), CONNECT_TO_BOTTOM),
                ((0, -1), CONNECT_TO_LEFT),
                ((0, 1), CONNECT_TO_RIGHT),
            ],
        ),
        (
            '|',
            vec![((-1, 0), CONNECT_TO_TOP), ((1, 0), CONNECT_TO_BOTTOM)],
        ),
        (
            '-',
            vec![((0, -1), CONNECT_TO_LEFT), ((0, 1), CONNECT_TO_RIGHT)],
        ),
        (
            'L',
            vec![((-1, 0), CONNECT_TO_TOP), ((0, 1), CONNECT_TO_RIGHT)],
        ),
        (
            'J',
            vec![((-1, 0), CONNECT_TO_TOP), ((0, -1), CONNECT_TO_LEFT)],
        ),
        (
            '7',
            vec![((1, 0), CONNECT_TO_BOTTOM), ((0, -1), CONNECT_TO_LEFT)],
        ),
        (
            'F',
            vec![((1, 0), CONNECT_TO_BOTTOM), ((0, 1), CONNECT_TO_RIGHT)],
        ),
    ]);

    let mut start_offsets = vec![];

    while !q.is_empty() {
        let mut next_q = vec![];
        for (coord, c) in q.clone() {
            let result = tile_mapping.get(&c).unwrap();
            for (offset, chars) in result {
                let valid_coord = get_valid_offset(coord, *offset, y_max, x_max, &seen);
                if let Some(next_coord) = valid_coord {
                    let next_char = grid[next_coord.0][next_coord.1];
                    if chars.contains(&next_char) {
                        if i == -1 {
                            start_offsets.push(*offset)
                        }
                        next_q.push((next_coord, next_char));
                    }
                }
            }
        }
        q = next_q;
        for (x, _) in &q {
            seen.insert(*x);
        }
        i += 1;
    }

    // replace start
    let replaced_start = {
        let found = tile_mapping
            .iter()
            .filter_map(|(char, mapping)| {
                if *char == 'S' {
                    None
                } else {
                    Some((char, [mapping[0].0, mapping[1].0]))
                }
            })
            .find(|(_, offsets)| offsets.iter().all(|x| start_offsets.contains(x)));

        *found.unwrap().0
    };

    let mut replaced_grid = grid.clone();
    replaced_grid[start.0][start.1] = replaced_start;

    (i, seen, replaced_grid)
}

fn get_inner(grid: Vec<Vec<char>>, loop_coors: BTreeSet<(usize, usize)>) -> i32 {
    let mut inside_loop = false;
    let mut inner = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if loop_coors.contains(&(y, x)) {
                // use pairs to find one open & one close for loop
                // ['F', '7', '|'] conects downards
                // ['J', 'L', '|'] connects upwards & also works here
                if ['F', '7', '|'].contains(c) {
                    inside_loop = !inside_loop;
                }
            } else if inside_loop {
                inner += 1;
            }
        }
    }

    inner
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start: (usize, usize) = (0, 0);
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'S' {
                start = (y, x);
            };
        }
    }

    let (answer1, loop_coords, replaced_grid) = traverse(start, input);
    let answer2 = get_inner(replaced_grid, loop_coords);
    println!("{}\n{}", answer1, answer2);
}
