use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use advent_of_code_2023_temp::util;

type Pulse = bool;
type Map = HashMap<String, (String, Vec<String>)>;

fn parse_input(input: &[String]) -> Map {
    let mut map: Map = HashMap::new();

    for line in input {
        let (module, dests) = line.split_once(" -> ").unwrap();
        let dests = dests.split(", ").map(|x| x.to_owned()).collect::<Vec<_>>();
        if module == "broadcaster" {
            map.insert(module.to_owned(), (module.to_owned(), dests));
        } else {
            let module_type = &module[0..1];
            let name = &module[1..];
            map.insert(name.to_owned(), (module_type.to_owned(), dests));
        }
    }

    map
}

fn get_periodics(map: &Map) -> Vec<String> {
    // theory: there is one Conjuction (A) that outputs to rx
    // in order for A to output low pulse to rx, it needs to have all inputs (Periodics) be high pulse
    // all of the Periodics are Conjuctions,
    // this functions find their name and return them
    // the lcm of the period of each Periodics is the answer to part 2

    let target = "rx";
    let before_target = map
        .iter()
        .filter(|(_, (module_type, dests))| {
            module_type == "&" && dests.contains(&target.to_owned())
        })
        .collect::<Vec<_>>();
    let periodics = if before_target.len() == 1 {
        map.iter()
            .filter_map(|(name, (module_type, dests))| {
                if module_type == "&" && dests.contains(&before_target[0].0.to_owned()) {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    periodics
}

fn solve(map: &Map) -> (i64, i64) {
    // name, is_on
    let mut states: HashMap<String, bool> =
        HashMap::from_iter(map.iter().filter_map(|(name, (_, _))| {
            if name == "broadcaster" {
                None
            } else {
                Some((name.to_owned(), (false)))
            }
        }));

    let mut q: Vec<(&String, Pulse)> = map
        .iter()
        .find_map(|(name, (_, dest))| {
            if name == "broadcaster" {
                Some(dest.iter().map(|x| (x, false)).collect())
            } else {
                None
            }
        })
        .unwrap();
    let start_q = q.clone();

    let mut low = 0i64;
    let mut high = 0i64;
    let mut i = 0;

    let periodics = get_periodics(map);
    let mut periodics_periods = vec![0i64; periodics.len()];

    'outer: loop {
        i += 1;
        if i <= 1000 {
            low += 1;
        }

        while !q.is_empty() {
            let mut next_q: Vec<(&String, Pulse)> = vec![];
            for (dest, pulse) in q {
                if i <= 1000 {
                    if pulse {
                        high += 1;
                    } else {
                        low += 1;
                    }
                }

                if let Some((module_type, dests)) = map.get(dest) {
                    let is_on = states.get(dest).unwrap();

                    if module_type == "%" {
                        if !pulse {
                            next_q.extend(dests.iter().map(|x| (x, !is_on)));
                            *states.get_mut(dest).unwrap() = !is_on;
                        }
                    } else {
                        let all_high = map
                            .iter()
                            .filter_map(|(name, (_, dests))| {
                                if dests.contains(dest) {
                                    Some(states.get(name).unwrap())
                                } else {
                                    None
                                }
                            })
                            .all(|x| *x);

                        if !all_high {
                            let found = periodics.iter().position(|x| x == dest);

                            if let Some(j) = found {
                                if periodics_periods[j] == 0 {
                                    periodics_periods[j] = i;
                                }

                                if periodics_periods.iter().all(|x| *x != 0) {
                                    break 'outer;
                                }
                            }
                        }

                        next_q.extend(dests.iter().map(|x| (x, !all_high)));
                        *states.get_mut(dest).unwrap() = !all_high;
                    }
                }
            }
            q = next_q;
        }

        if periodics.is_empty() && i == 1000 {
            break;
        }
        q = start_q.clone();
    }

    let answer2 = if periodics.is_empty() {
        0
    } else {
        util::lcm(periodics_periods)
    };
    (low * high, answer2)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let map = parse_input(&input);
    let (answer1, answer2) = solve(&map);
    println!("{}\n{}", answer1, answer2);
}
