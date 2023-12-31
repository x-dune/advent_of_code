use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    io::{self, BufRead},
};

use rand::Rng;

type Graph<'a> = HashMap<String, HashSet<String>>;

fn component_size(graph: &Graph, start: &str) -> usize {
    let mut seen = HashSet::new();
    let mut q = vec![start];
    while let Some(node) = q.pop() {
        if !seen.insert(node) {
            continue;
        }
        if let Some(ns) = graph.get(node) {
            for n in ns {
                q.push(n);
            }
        }
    }
    seen.len()
}

fn parse_input(input: &[String]) -> Graph {
    let mut graph: Graph = HashMap::new();

    input.iter().for_each(|e| {
        let (component, neighbours) = e.split_once(": ").unwrap();
        for n in neighbours.split(' ') {
            graph
                .entry(component.to_owned())
                .or_default()
                .insert(n.to_owned());
            graph
                .entry(n.to_owned())
                .or_default()
                .insert(component.to_owned());
        }
    });

    graph
}

fn bfs(graph: &Graph, start: &str, target: &str) -> Option<Vec<String>> {
    let mut q = VecDeque::from([(start, vec![start.to_owned()])]);
    let mut seen = HashSet::from([start]);

    while let Some((current, path)) = q.pop_front() {
        for n in &graph[current] {
            let nr: &str = n.as_ref();
            if !seen.contains(nr) {
                let mut next_path = path.clone();
                next_path.push(n.to_owned());
                if nr == target {
                    return Some(next_path);
                }
                seen.insert(nr);
                q.push_back((nr, next_path));
            }
        }
    }
    None
}

fn solve(graph: &Graph) -> Option<usize> {
    let mut count = HashMap::new();
    let keys = graph.keys().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let a = keys[rng.gen_range(0..graph.len() - 1)];
        let b = keys[rng.gen_range(0..graph.len() - 1)];
        let path = bfs(graph, a, b);
        if let Some(path) = path {
            for n in path {
                if n != *a && n != *b {
                    *count.entry(n).or_insert(0) += 1;
                }
            }
        }
    }

    let mut sorted = count.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|(_, v)| cmp::Reverse(*v));

    let mut cut_graph = graph.clone();
    for &(n, _) in sorted.iter().take(6) {
        cut_graph.remove(n);
    }

    for _ in 0..10 {
        let c1 = component_size(&cut_graph, keys[rng.gen_range(0..graph.len() - 1)]);
        let c2 = component_size(&cut_graph, keys[rng.gen_range(0..graph.len() - 1)]);

        if c1 != c2 {
            return Some(c1 * c2);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let graph = parse_input(&input);
    let answer = solve(&graph);

    println!("{}", answer.unwrap_or(0));
}
