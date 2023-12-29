use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

type Coord = (usize, usize);

const OFFSETS: [(isize, isize, char); 4] = [(-1, 0, 'v'), (1, 0, '^'), (0, -1, '>'), (0, 1, '<')];

fn get_neighbours((y, x): Coord, grid: &[Vec<char>], part2: bool) -> Vec<Coord> {
    OFFSETS
        .iter()
        .filter_map(|(dy, dx, forbidden)| {
            if let (Some(next_y), Some(next_x)) =
                (y.checked_add_signed(*dy), x.checked_add_signed(*dx))
            {
                let next = (next_y, next_x);
                if next.0 < grid.len()
                    && next.1 < grid[0].len()
                    && grid[next.0][next.1] != '#'
                    && (part2 || grid[next.0][next.1] != *forbidden)
                {
                    return Some(next);
                }
            }
            None
        })
        .collect()
}

fn solve1(grid: &[Vec<char>]) -> usize {
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    let mut q = vec![(HashSet::from([(0, 0)]), (0, 1))];

    let mut max_dist = 0;
    while let Some((path, pos)) = q.pop() {
        let current_path = dist.get(&pos).unwrap_or(&0);
        if path.len() < *current_path {
            continue;
        }

        if pos == (grid.len() - 1, grid[0].len() - 2) {
            max_dist = max_dist.max(path.len() - 1);
        }

        for next in get_neighbours(pos, grid, false) {
            if !path.contains(&next) {
                let entry = dist.entry(next).or_insert(0);
                let next_cost = path.len() + 1;
                if next_cost > *entry {
                    let mut next_path = path.clone();
                    next_path.insert(next);
                    q.push((next_path, next));
                }
            }
        }
    }

    max_dist
}

fn solve2(grid: &[Vec<char>]) -> u32 {
    let mut neighbours_map = HashMap::new();
    let mut hallways = vec![];

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c != '#' {
                let neighbours = get_neighbours((y, x), grid, true)
                    .into_iter()
                    .map(|(ny, nx)| (ny, nx, 1))
                    .collect::<Vec<_>>();
                if neighbours.len() == 2 {
                    hallways.push((y, x));
                }
                neighbours_map.insert((y, x), neighbours);
            }
        })
    });

    hallways.iter().for_each(|pos| {
        let neighbours = neighbours_map.remove(pos).unwrap();
        let (y1, x1, d1) = neighbours[0];
        let (y2, x2, d2) = neighbours[1];

        let n1 = neighbours_map.get_mut(&(y1, x1)).unwrap();
        if let Some(i) = n1.iter().position(|&(y, x, _)| *pos == (y, x)) {
            n1[i] = (y2, x2, d1 + d2);
        };

        let n2 = neighbours_map.get_mut(&(y2, x2)).unwrap();
        if let Some(i) = n2.iter().position(|&(y, x, _)| *pos == (y, x)) {
            n2[i] = (y1, x1, d1 + d2);
        }
    });

    let target = (grid.len() - 1, grid[0].len() - 2);

    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    dfs(&neighbours_map, &mut seen, (0, 1), target, 0)
}

fn dfs(
    graph: &HashMap<Coord, Vec<(usize, usize, u32)>>,
    seen: &mut Vec<Vec<bool>>,
    start: Coord,
    target: Coord,
    curr_dist: u32,
) -> u32 {
    if start == target {
        return curr_dist;
    }
    let mut max_dist = 0;
    for &(y, x, d) in &graph[&start] {
        if !seen[y][x] {
            seen[y][x] = true;
            let dist = dfs(graph, seen, (y, x), target, curr_dist + d);
            max_dist = max_dist.max(dist);
            seen[y][x] = false;
        }
    }
    max_dist
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = solve1(&input);
    let answer2 = solve2(&input);

    println!("{}\n{:?}", answer1, answer2);
}
