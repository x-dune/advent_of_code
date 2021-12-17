use std::cmp::max;

fn parse_input() -> ((i32, i32), (i32, i32)) {
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

    // x range, y range
    (ranges[0], ranges[1])
}

fn step((x, y): (i32, i32), (dx, dy): (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let next_pos = (x + dx, y + dy);

    let next_dx = if dx > 0 {
        dx - 1
    } else if dx < 0 {
        dx + 1
    } else {
        0
    };
    let next_dy = dy - 1;

    (next_pos, (next_dx, next_dy))
}

fn try_velocity(start_vel: (i32, i32), (goal_x, goal_y): ((i32, i32), (i32, i32))) -> Option<i32> {
    let (range_x, range_y) = (goal_x.0..=goal_x.1, goal_y.0..=goal_y.1);
    let mut pos = (0, 0);
    let mut vel = start_vel;
    let mut highest_pos_y = 0;

    while !(range_x.contains(&pos.0) && range_y.contains(&pos.1)) {
        let (next_pos, next_vel) = step(pos, vel);
        pos = next_pos;
        vel = next_vel;
        highest_pos_y = max(highest_pos_y, pos.1);

        if (vel.0 == 0 && !range_x.contains(&pos.0)) || (vel.1 < 0 && pos.1 < goal_y.0) {
            return None;
        }
    }

    return Some(highest_pos_y);
}

fn main() {
    let goal = parse_input();

    let max_heights = (-250..=250)
        .flat_map(|x| (-250..=250).filter_map(move |y| try_velocity((x, y), goal)))
        .collect::<Vec<_>>();

    let answer1 = max_heights.iter().max().unwrap();
    let answer2 = max_heights.iter().count();

    println!("{}\n{}", answer1, answer2);
}
