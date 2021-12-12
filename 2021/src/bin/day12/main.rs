use std::{
    collections::{HashMap, VecDeque},
    vec,
};

fn get_paths_1(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut q: VecDeque<Vec<String>> = VecDeque::new();
    q.push_back(vec!["start".to_string()]);

    let mut paths = vec![];
    while q.len() > 0 {
        let traversed_paths = q.pop_front().unwrap();
        let current_node = traversed_paths.last().unwrap();

        if current_node == "end" {
            paths.push(traversed_paths.join(","));
            continue;
        } else {
            let neighbours = &graph[current_node];

            for n in neighbours {
                if n.to_lowercase().to_owned() != *n || !traversed_paths.contains(n) {
                    let mut next_path = traversed_paths.clone();
                    next_path.push(n.to_string());
                    q.push_back(next_path);
                }
            }
        }
    }

    return paths;
}

fn has_revisit_small_cave(path: &Vec<String>) -> bool {
    // has already revisited small cave in this run
    let small_cave_revisit_count = path
        .iter()
        .filter(|&x| x.to_lowercase() == *x && x != &"start".to_string())
        .fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        });

    let bool = small_cave_revisit_count
        .values()
        .filter(|&x| x > &1)
        .count()
        == 1;

    bool
}

fn get_paths_2(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut q = VecDeque::new();
    q.push_back(vec!["start".to_string()]);

    let mut paths = vec![];
    while q.len() > 0 {
        // println!("{:?}", q);
        let traversed_paths = q.pop_front().unwrap();
        let current_node = traversed_paths.last().unwrap();

        if current_node == "end" {
            paths.push(traversed_paths.join(","));
            continue;
        } else {
            let neighbours = &graph[current_node];

            for n in neighbours {
                if n == "start" {
                    continue;
                }

                let mut next_path = traversed_paths.clone();
                next_path.push(n.to_string());

                if current_node == "d" {
                    // println!("{} {:?} {} {}", n, next_path, traversed_paths.contains(n),);
                }
                if n.to_lowercase().to_owned() != *n {
                    q.push_back(next_path);
                } else {
                    // small cave handling
                    if !traversed_paths.contains(n) {
                        q.push_back(next_path);
                    } else if !has_revisit_small_cave(&traversed_paths) {
                        q.push_back(next_path);
                    } else {
                        // about to visit small cave twice but already used up has_visited_small_cave_twice
                        continue;
                    }
                }
            }
        }
    }

    return paths;
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|s| {
            let nodes = s.split("-").collect::<Vec<_>>();
            (nodes[0].to_string(), nodes[1].to_string())
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a.clone()).or_insert(vec![]).push(b.clone());
            acc.entry(b).or_insert(vec![]).push(a);
            acc
        });

    let answer1 = get_paths_1(&input);
    let answer2 = get_paths_2(&input);

    println!("{:?}", answer1.len());
    println!("{:?}", answer2.len());
}
