use std::collections::HashMap;

fn get_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    traversed: &[&'a str],
    has_revisited_small_cave: bool,
    current: &'a str,
) -> Vec<(Vec<&'a str>, bool)> {
    if current == "start" && !traversed.is_empty() {
        return vec![];
    }

    let mut next_traversed = traversed.to_owned();
    next_traversed.push(current);
    if current == "end" {
        return vec![(next_traversed, has_revisited_small_cave)];
    }

    let mut next_has_revisited_small_cave = has_revisited_small_cave;
    if current.to_lowercase() == current && traversed.contains(&current) {
        if has_revisited_small_cave {
            return vec![];
        } else {
            next_has_revisited_small_cave = true;
        }
    }

    let neighbours = &graph[current];
    neighbours
        .iter()
        .flat_map(|n| get_paths(graph, &next_traversed, next_has_revisited_small_cave, n))
        .collect()
}

fn main() {
    let graph: HashMap<&str, Vec<&str>> = include_str!("input.txt")
        .lines()
        .map(|s| s.split_once('-').unwrap())
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().push(b);
            acc.entry(b).or_default().push(a);
            acc
        });

    let paths = get_paths(&graph, &[], false, "start");
    let answer1 = paths
        .iter()
        .filter(|(_, has_revisited_small_cave)| !has_revisited_small_cave)
        .count();
    let answer2 = paths.len();

    println!("{} {}", answer1, answer2);
}
