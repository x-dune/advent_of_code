use std::collections::{HashMap, HashSet};

// x, y, z
type Vec3 = (i32, i32, i32);

fn parse_input() -> Vec<Vec<Vec3>> {
    include_str!("input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    let pos = l
                        .split(',')
                        .map(|a| a.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    (pos[0], pos[1], pos[2])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn has_match(resolved_beacons: &HashSet<Vec3>, beacons: &[Vec3]) -> Option<Vec3> {
    let translation = resolved_beacons
        .iter()
        .flat_map(|pos1| {
            beacons
                .iter()
                .map(move |pos2| (pos1.0 - pos2.0, pos1.1 - pos2.1, pos1.2 - pos2.2))
        })
        .fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        });

    translation.iter().find(|(_, &v)| v >= 12).map(|(&k, _)| k)
}

fn resolve_scanners(scanners: &[Vec<Vec3>]) -> (HashSet<Vec3>, Vec<Vec3>) {
    let mut resolved_beacons = scanners[0].iter().fold(HashSet::new(), |mut acc, curr| {
        acc.insert(*curr);
        acc
    });
    let mut resolved_scanners = vec![(0, 0, 0)];
    let mut rest = scanners.to_owned();
    rest.remove(0);

    while !rest.is_empty() {
        let mut to_remove = None;
        'outer: for (i, scanner) in rest.iter().enumerate() {
            for rotated_scanner in get_all_rotations(scanner) {
                if let Some(d) = has_match(&resolved_beacons, &rotated_scanner) {
                    resolved_beacons.extend(
                        rotated_scanner
                            .iter()
                            .map(|pos| (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2)),
                    );

                    resolved_scanners.push(d);

                    to_remove = Some(i);
                    break 'outer;
                }
            }
        }

        if let Some(i) = to_remove {
            rest.remove(i);
        } else {
            unreachable!()
        }
    }

    (resolved_beacons, resolved_scanners)
}

static ROTATION_ANGLE: [u32; 4] = [90, 180, 270, 360];

fn get_all_rotations(pos: &[Vec3]) -> Vec<Vec<Vec3>> {
    let all_rotations = ROTATION_ANGLE
        .iter()
        .flat_map(|x| {
            ROTATION_ANGLE
                .iter()
                .flat_map(move |y| ROTATION_ANGLE.iter().map(move |z| (*x, *y, *z)))
        })
        .collect::<HashSet<_>>();

    let mut rotated_scanners: HashMap<(u32, u32, u32), Vec<Vec3>> = HashMap::new();

    for vec in pos {
        all_rotations.iter().for_each(|&angles| {
            rotated_scanners
                .entry(angles)
                .or_default()
                .push(rotate(*vec, angles))
        });
    }

    rotated_scanners.into_values().collect::<Vec<_>>()
}

fn rotate(pos: Vec3, (rx, ry, rz): (u32, u32, u32)) -> Vec3 {
    let (mut x, mut y, mut z) = pos;

    match rx {
        90 => {
            let temp_y = y;
            y = z;
            z = -temp_y
        }
        180 => {
            y = -y;
            z = -z
        }
        270 => {
            let temp_y = y;
            y = -z;
            z = temp_y
        }
        _ => (),
    }

    match ry {
        90 => {
            let temp_x = x;
            x = z;
            z = -temp_x
        }
        180 => {
            x = -x;
            z = -z
        }
        270 => {
            let temp_x = x;
            x = -z;
            z = temp_x
        }
        _ => (),
    }

    match rz {
        90 => {
            let temp_y = y;
            y = x;
            x = -temp_y
        }
        180 => {
            y = -y;
            x = -x
        }
        270 => {
            let temp_y = y;
            y = -x;
            x = temp_y
        }
        _ => (),
    }

    (x, y, z)
}

fn main() {
    let scanners = parse_input();

    let (resolved_beacons, resolved_scanners) = resolve_scanners(&scanners);
    println!("{}", &resolved_beacons.len());

    let manhattan_distances = resolved_scanners
        .iter()
        .flat_map(|a| {
            resolved_scanners.iter().filter_map(move |b| {
                if a == b {
                    None
                } else {
                    Some((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
                }
            })
        })
        .collect::<Vec<_>>();

    let answer2 = manhattan_distances.iter().max().unwrap();

    println!("{}", answer2);
}
