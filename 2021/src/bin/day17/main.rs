use std::ops::RangeInclusive;

fn parse_input() -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (_, x_y) = include_str!("input.txt").trim().split_once(": ").unwrap();
    let ranges = x_y
        .split(", ")
        .map(|s| {
            let range = (&s[2..])
                .split("..")
                .map(|r| r.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (*range.iter().min().unwrap(), *range.iter().max().unwrap())
        })
        .collect::<Vec<_>>();

    // range_x, range_y
    (ranges[0].0..=ranges[0].1, ranges[1].0..=ranges[1].1)
}

fn try_velocity(
    start_vel: (i32, i32),
    range_x: &RangeInclusive<i32>,
    range_y: &RangeInclusive<i32>,
) -> Option<i32> {
    let mut pos = (0, 0);
    let (mut dx, mut dy) = start_vel;
    let mut highest_pos_y = 0;

    loop {
        pos = (pos.0 + dx, pos.1 + dy);
        dx -= dx.signum();
        dy -= 1;

        if pos.1 > highest_pos_y {
            highest_pos_y = pos.1;
        }

        if range_x.contains(&pos.0) && range_y.contains(&pos.1) {
            return Some(highest_pos_y);
        } else if (dx == 0 && !range_x.contains(&pos.0)) || (dy < 0 && pos.1 < *range_y.start()) {
            return None;
        }
    }
}

fn main() {
    let (range_x, range_y) = parse_input();
    let max_heights = (0..=*range_x.end())
        .flat_map(|dx| (*range_y.start()..=1000).map(move |dy| (dx, dy)))
        .filter_map(|v| try_velocity(v, &range_x, &range_y))
        .collect::<Vec<_>>();

    let answer1 = max_heights.iter().max().unwrap();
    let answer2 = max_heights.iter().count();

    println!("{}\n{}", answer1, answer2);
}
