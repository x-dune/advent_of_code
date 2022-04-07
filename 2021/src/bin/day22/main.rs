fn parse_input() -> Vec<(bool, [(i64, i64); 3])> {
    include_str!("input.txt")
        .lines()
        .map(|a| {
            let (on_off, rest) = a.split_once(' ').unwrap();

            let cube = rest
                .split(',')
                .map(|b| {
                    let (low, high) = &b[2..].split_once("..").unwrap();
                    (low.parse::<i64>().unwrap(), high.parse::<i64>().unwrap())
                })
                .collect::<Vec<_>>();

            (on_off == "on", cube.try_into().unwrap())
        })
        .collect()
}

fn overlap_axis((low1, high1): (i64, i64), (low2, high2): (i64, i64)) -> Option<(i64, i64)> {
    if high1 >= low2 && high2 >= low1 {
        Some((low1.max(low2), high1.min(high2)))
    } else {
        None
    }
}

fn overlap_cube(c1: &[(i64, i64); 3], c2: &[(i64, i64); 3]) -> Option<[(i64, i64); 3]> {
    let cube = (0..3)
        .map_while(|i| overlap_axis(c1[i], c2[i]))
        .collect::<Vec<_>>();

    if cube.len() == 3 {
        Some(cube[..3].try_into().unwrap())
    } else {
        None
    }
}

fn volume(cube: &[(i64, i64); 3]) -> i64 {
    cube.iter().map(|(low, high)| high - low + 1).product()
}

fn total_volume(cubes: &[(bool, [(i64, i64); 3])]) -> i64 {
    let mut added_cubes = vec![];
    let mut removed_cubes = vec![];

    for &(on, cube) in cubes {
        let overlap = added_cubes.iter().filter_map(|c| overlap_cube(&cube, c));
        let removed_overlap = removed_cubes
            .iter()
            .filter_map(|c| overlap_cube(&cube, c))
            .collect::<Vec<_>>();

        removed_cubes.extend(overlap);
        added_cubes.extend(removed_overlap);
        if on {
            added_cubes.push(cube);
        }
    }

    let total_volume =
        added_cubes.iter().map(volume).sum::<i64>() - removed_cubes.iter().map(volume).sum::<i64>();

    total_volume
}

fn main() {
    let input = parse_input();

    let init_zone = input
        .iter()
        .filter_map(|(on, c)| {
            overlap_cube(c, &[(-50, 50), (-50, 50), (-50, 50)]).map(|new_cube| (*on, new_cube))
        })
        .collect::<Vec<_>>();
    let answer1 = total_volume(&init_zone);
    println!("{}", answer1);
    let answer2 = total_volume(&input);
    println!("{}", answer2);
}
