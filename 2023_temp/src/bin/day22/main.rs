use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    io::{self, BufRead},
};

type Coord = (i32, i32, i32);

fn parse_input(input: &[String]) -> Vec<(Coord, Coord)> {
    input
        .iter()
        .map(|x| {
            let coords: Vec<(i32, i32, i32)> = x
                .split('~')
                .map(|y| {
                    let coord = y
                        .split(',')
                        .map(|z| z.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    (coord[0], coord[1], coord[2])
                })
                .collect();
            (coords[0], coords[1])
        })
        .collect()
}

#[derive(Debug, Clone)]
struct State {
    top: Vec<(i32, i32, i32)>,
    bottom: Vec<(i32, i32, i32)>,
    supported_by: Vec<i32>,
    settled: bool,
}

type Simulation = BTreeMap<i32, State>;

fn simulate(coords: &[(Coord, Coord)]) -> (usize, i32) {
    let mut simulation: Simulation = BTreeMap::new();

    for (i, ((x1, y1, z1), (x2, y2, z2))) in coords.iter().enumerate() {
        let mut bottom = BTreeSet::new();

        // add bottom
        for x in 0..=(x2 - x1) {
            for y in 0..=(y2 - y1) {
                bottom.insert((x1 + x, y1 + y, *z1));
            }
        }

        let bottom = bottom.into_iter().collect::<Vec<_>>();

        let top = if z2 > z1 {
            bottom.iter().map(|(x, y, _)| (*x, *y, *z2)).collect()
        } else {
            bottom.clone()
        };

        simulation.insert(
            i as i32 + 1,
            State {
                top,
                bottom,
                supported_by: vec![],
                settled: false,
            },
        );
    }

    let mut q = simulation
        .clone()
        .into_iter()
        .filter(|(_, x)| !x.settled)
        .collect::<Vec<_>>();

    q.sort_by(|(_, a), (_, b)| a.bottom[0].2.cmp(&b.bottom[0].2));

    while !q.is_empty() {
        for (i, state) in q {
            if state.bottom[0].2 == 1 {
                let entry = simulation.get_mut(&i).unwrap();
                entry.supported_by.push(0);
                entry.settled = true;
            } else {
                let next_bottom = state
                    .bottom
                    .iter()
                    .map(|(x, y, z)| (*x, *y, z - 1))
                    .collect::<Vec<_>>();

                let contact = simulation
                    .iter()
                    .filter_map(|(k, x)| {
                        if x.settled && x.top.iter().any(|x| next_bottom.contains(x)) {
                            Some(*k)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let entry = simulation.get_mut(&i).unwrap();
                if !contact.is_empty() {
                    entry.supported_by.extend(contact);
                    entry.settled = true;
                } else {
                    let next_top = state
                        .top
                        .iter()
                        .map(|(x, y, z)| (*x, *y, z - 1))
                        .collect::<Vec<_>>();

                    entry.bottom = next_bottom;
                    entry.top = next_top;
                }
            }
        }

        q = simulation
            .clone()
            .into_iter()
            .filter(|(_, x)| !x.settled)
            .collect::<Vec<_>>();
        q.sort_by(|(_, a), (_, b)| a.bottom[0].2.cmp(&b.bottom[0].2));
    }

    let mut cannot_disintegrate = HashSet::new();
    for v in simulation.values() {
        if v.supported_by.len() == 1 && v.supported_by[0] != 0 {
            cannot_disintegrate.insert(v.supported_by[0]);
        }
    }

    let answer1 = coords.len() - cannot_disintegrate.len();

    let mut answer2 = 0;

    for k in &cannot_disintegrate {
        let mut q2 = vec![k];
        let mut chain_reaction = -1; // start at -1 to account for initial block which is not counted
        let mut seen = HashSet::new();
        while let Some(k1) = q2.pop() {
            if seen.contains(&k1) {
                continue;
            }
            chain_reaction += 1;

            q2.extend(simulation.iter().filter_map(|(k2, v2)| {
                if v2.supported_by.contains(k1)
                    && v2
                        .supported_by
                        .iter()
                        .filter(|x| !seen.contains(*x))
                        .collect::<Vec<_>>()
                        .len()
                        == 1
                {
                    Some(k2)
                } else {
                    None
                }
            }));
            seen.insert(k1);
        }
        answer2 += chain_reaction;
    }

    (answer1, answer2)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();
    let coords = parse_input(&input);

    let (answer1, answer2) = simulate(&coords);

    println!("{}\n{}", answer1, answer2);
}
