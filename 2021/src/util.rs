pub fn input_as_string(input: &str) -> &str {
    input.trim()
}

pub fn input_as_lines(input: &str) -> Vec<&str> {
    return input.trim().split('\n').collect();
}

pub fn input_as_ints_from_lines(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
}

pub fn input_as_ints_from_list(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
}

// Transpose a 2D array
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
