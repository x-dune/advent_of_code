pub fn input_as_ints(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}
