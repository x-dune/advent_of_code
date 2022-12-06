use std::collections::HashSet;

#[derive(Debug, Clone)]
enum SNumber {
    Val(i32),
    Pair(Box<SNumber>, Box<SNumber>),
}

impl SNumber {
    fn new_pair(left: SNumber, right: SNumber) -> SNumber {
        SNumber::Pair(Box::new(left), Box::new(right))
    }
}

fn parse(s: &str) -> (SNumber, &str) {
    let current = s.chars().next().unwrap();
    if current == '[' {
        let (left, rest_left) = parse(&s[1..]);
        let (right, rest_right) = parse(&rest_left[1..]);
        (
            SNumber::Pair(Box::new(left), Box::new(right)),
            &rest_right[1..],
        )
    } else {
        let n = current.to_digit(10).unwrap() as i32;
        (SNumber::Val(n), &s[1..])
    }
}

fn add_left(snum: &SNumber, to_add: &Option<SNumber>) -> SNumber {
    match (snum, to_add) {
        (_, None) => snum.clone(),
        (SNumber::Val(a), Some(SNumber::Val(b))) => SNumber::Val(a + b),
        (SNumber::Pair(l, r), _) => SNumber::Pair(Box::new(add_left(l, to_add)), r.clone()),
        _ => unreachable!(),
    }
}

fn add_right(snum: &SNumber, to_add: &Option<SNumber>) -> SNumber {
    match (snum, to_add) {
        (_, None) => snum.clone(),
        (SNumber::Val(a), Some(SNumber::Val(b))) => SNumber::Val(a + b),
        (SNumber::Pair(l, r), _) => SNumber::Pair(l.clone(), Box::new(add_right(r, to_add))),
        _ => unreachable!(),
    }
}

fn try_explode(snum: &SNumber, depth: u32) -> (bool, SNumber, Option<SNumber>, Option<SNumber>) {
    if let SNumber::Pair(l, r) = snum {
        if depth == 4 {
            if let (SNumber::Val(a), SNumber::Val(b)) = (*l.clone(), *r.clone()) {
                return (
                    true,
                    SNumber::Val(0),
                    Some(SNumber::Val(a)),
                    Some(SNumber::Val(b)),
                );
            }
        } else {
            let (exploded, exploded_snum_l, left_add, right_add) = try_explode(l, depth + 1);
            if exploded {
                return (
                    true,
                    SNumber::Pair(Box::new(exploded_snum_l), Box::new(add_left(r, &right_add))),
                    left_add,
                    None,
                );
            }
            let (exploded, exploded_snum_r, left_add, right_add) = try_explode(r, depth + 1);
            if exploded {
                return (
                    true,
                    SNumber::new_pair(add_right(l, &left_add), exploded_snum_r),
                    None,
                    right_add,
                );
            }
        }
    }
    (false, snum.clone(), None, None)
}

fn try_split(snum: &SNumber) -> (bool, SNumber) {
    match snum {
        SNumber::Val(a) => {
            if *a >= 10 {
                return (
                    true,
                    SNumber::new_pair(
                        SNumber::Val((*a as f64 / 2.0).floor() as i32),
                        SNumber::Val((*a as f64 / 2.0).ceil() as i32),
                    ),
                );
            }
        }
        SNumber::Pair(l, r) => {
            let (splitted, new_snum) = try_split(l);
            if splitted {
                return (true, SNumber::new_pair(new_snum, *r.clone()));
            }

            let (splitted, new_snum) = try_split(r);
            if splitted {
                return (true, SNumber::new_pair(*l.clone(), new_snum));
            }
        }
    }

    (false, snum.clone())
}

fn magnitude(snum: &SNumber) -> i32 {
    match snum {
        SNumber::Val(a) => *a,
        SNumber::Pair(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn calculate_magnitude(snums: &[SNumber]) -> i32 {
    let mut snum = snums[0].clone();

    for item in snums.iter().skip(1) {
        snum = SNumber::new_pair(snum, item.clone());

        loop {
            let (exploded, next_snum, _, _) = try_explode(&snum, 0);

            if exploded {
                snum = next_snum;
                continue;
            }

            let (splitted, next_snum) = try_split(&snum);

            if splitted {
                snum = next_snum;
                continue;
            }

            break;
        }
    }

    magnitude(&snum)
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|s| parse(s).0)
        .collect::<Vec<_>>();

    let answer1 = calculate_magnitude(&input);
    println!("{}", answer1);

    let mut answer2 = 0;
    let permutations = (0..input.len())
        .flat_map(|n1| {
            (0..input.len()).flat_map(move |n2| {
                if n1 == n2 {
                    vec![]
                } else {
                    vec![(n1, n2), (n2, n1)]
                }
            })
        })
        .collect::<HashSet<_>>();
    for (n1, n2) in permutations {
        let magnitude = calculate_magnitude(&[input[n1].clone(), input[n2].clone()]);

        if magnitude > answer2 {
            answer2 = magnitude;
        }
    }
    println!("{}", answer2);
}
