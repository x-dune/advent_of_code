use std::io::{self, BufRead};

type Vec3 = (i64, i64, i64);

fn parse_input(input: &[String]) -> Vec<(Vec3, Vec3)> {
    input
        .iter()
        .map(|line| {
            let pos_vec = line
                .split('@')
                .map(|x| {
                    let nums = x
                        .split(',')
                        .map(|y| {
                            return y.trim().parse::<i64>().unwrap();
                        })
                        .collect::<Vec<_>>();

                    (nums[0], nums[1], nums[2])
                })
                .collect::<Vec<_>>();

            (pos_vec[0], pos_vec[1])
        })
        .collect()
}

fn did_intersect(
    ((x1, y1, _), (dx1, dy1, _)): (Vec3, Vec3),
    ((x3, y3, _), (dx3, dy3, _)): (Vec3, Vec3),
    bounds: (f64, f64),
) -> bool {
    let (x2, y2) = (x1 + dx1, y1 + dy1);
    let (x4, y4) = (x3 + dx3, y3 + dy3);

    let a1 = (y1 - y2) as f64;
    let b1 = (x2 - x1) as f64;
    let c1 = -((x1 * y2) - (x2 * y1)) as f64;
    let a2 = (y3 - y4) as f64;
    let b2 = (x4 - x3) as f64;
    let c2 = -((x3 * y4) - (x4 * y3)) as f64;

    let d = a1 * b2 - b1 * a2;
    let dx = c1 * b2 - b1 * c2;
    let dy = a1 * c2 - c1 * a2;

    if d != 0. {
        let ix = dx / d;
        let iy = dy / d;

        if ix >= bounds.0
            && ix <= bounds.1
            && iy >= bounds.0
            && iy <= bounds.1
            && (ix > x1 as f64) == (x2 > x1)
            && (iy > y1 as f64) == (y2 > y1)
            && (ix > x3 as f64) == (x4 > x3)
            && (iy > y3 as f64) == (y4 > y3)
        {
            return true;
        }
    }
    false
}

fn get_intersections(hailstones: &[(Vec3, Vec3)], bounds: (f64, f64)) -> i64 {
    let mut intersections = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if did_intersect(hailstones[i], hailstones[j], bounds) {
                intersections += 1;
            }
        }
    }
    intersections
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let hailstones = parse_input(&input);
    let answer1 = get_intersections(&hailstones, (200000000000000., 400000000000000.));

    println!("{}", answer1);
}
