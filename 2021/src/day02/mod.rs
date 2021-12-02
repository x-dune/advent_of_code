use crate::util;

pub fn parsed_input() -> Vec<(&'static str, i32)> {
    return util::input_as_lines(include_str!("input.txt"))
        .iter()
        .map(|x| {
            let mut split = x.split(' ');
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i32)>>();
}

pub fn solution1() -> i32 {
    let input = parsed_input();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut i = 0;
    while i < input.len() {
        let (direction, magnitude) = input[i];
        if direction == "forward" {
            horizontal += magnitude;
        } else if direction == "up" {
            depth -= magnitude;
        } else if direction == "down" {
            depth += magnitude;
        }

        i += 1;
    }

    return horizontal * depth;
}

pub fn solution2() -> i32 {
    let input = parsed_input();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut i = 0;
    while i < input.len() {
        let (direction, magnitude) = input[i];
        if direction == "forward" {
            horizontal += magnitude;
            depth += aim * magnitude;
        } else if direction == "up" {
            aim -= magnitude;
        } else if direction == "down" {
            aim += magnitude;
        }

        i += 1;
    }

    return horizontal * depth;
}
