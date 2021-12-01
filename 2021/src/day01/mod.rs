use crate::util;

pub fn solution1() -> i32 {
    let input = util::input_as_ints(include_str!("input.txt"));

    let mut counter = 0;
    let mut i = 0;
    while i < input.len() - 1 {
        if input[i + 1] > input[i] {
            counter += 1;
        }
        i += 1;
    }
    return counter;
}

pub fn solution2() -> i32 {
    let input = util::input_as_ints(include_str!("input.txt"));

    let mut counter = 0;
    let mut i = 0;
    /*
    We're not actually going to sum the sliding window, we're only going to compare the elements from both group that
    don't overlap which is the first element of the first group and the last element of the second group
    */
    while i < input.len() - 3 {
        if input[i + 3] > input[i] {
            counter += 1;
        }
        i += 1;
    }
    return counter;
}
