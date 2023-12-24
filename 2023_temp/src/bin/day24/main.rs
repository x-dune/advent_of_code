use std::io::{self, BufRead};

use z3::ast::Ast;

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

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x = z3::ast::Int::new_const(&ctx, "x");
    let y = z3::ast::Int::new_const(&ctx, "y");
    let z = z3::ast::Int::new_const(&ctx, "z");
    let vx = z3::ast::Int::new_const(&ctx, "vx");
    let vy = z3::ast::Int::new_const(&ctx, "vy");
    let vz = z3::ast::Int::new_const(&ctx, "vz");

    for (i, hs) in hailstones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&ctx, hs.0 .0);
        let va = z3::ast::Int::from_i64(&ctx, hs.1 .0);
        let b = z3::ast::Int::from_i64(&ctx, hs.0 .1);
        let vb = z3::ast::Int::from_i64(&ctx, hs.1 .1);
        let c = z3::ast::Int::from_i64(&ctx, hs.0 .2);
        let vc = z3::ast::Int::from_i64(&ctx, hs.1 .2);

        let t = z3::ast::Int::new_const(&ctx, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&ctx, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

    if solver.check() == z3::SatResult::Sat {
        if let Some(m) = solver.get_model() {
            println!("{}", m.eval(&(x + y + z), true).unwrap());
        } else {
            println!("Failed to get answer 2");
        }
    } else {
        println!("Failed to get answer 2");
    }
}
