use std::io::{self, BufRead};

fn hash(s: &str) -> usize {
    let mut curr = 0;
    for c in s.chars() {
        curr += c as u8 as usize;
        curr *= 17;
        curr %= 256;
    }
    curr
}

type Lens = (String, usize);
fn hash_map(s: &str, boxes: &mut [Vec<Lens>]) {
    if s.contains('=') {
        let (label, focal_length) = s.split_once('=').unwrap();
        let dest = hash(label);
        let next_lens = (label.to_owned(), focal_length.parse::<usize>().unwrap());

        if let Some(pos) = boxes[dest].iter().position(|(l, _)| l == label) {
            boxes[dest][pos] = next_lens;
        } else {
            boxes[dest].push(next_lens);
        }
    } else {
        let (label, _) = s.split_once('-').unwrap();
        let dest = hash(label);

        boxes[dest] = boxes[dest]
            .clone()
            .into_iter()
            .filter(|(l, _)| l != label)
            .collect::<Vec<_>>();
    }
}

fn get_focusing_power(boxes: &[Vec<Lens>]) -> usize {
    let mut result = 0;
    for (i, ls) in boxes.iter().enumerate() {
        for (j, (_, focal_length)) in ls.iter().enumerate() {
            result += (i + 1) * (j + 1) * focal_length;
        }
    }
    result
}

fn solve_part2(input: &[&str]) -> usize {
    let mut boxes = vec![vec![]; 256];
    input.iter().for_each(|x| hash_map(x, &mut boxes));
    get_focusing_power(&boxes)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();
    let parsed = input[0].split(',').collect::<Vec<_>>();

    let answer1 = parsed.iter().map(|x| hash(x)).sum::<usize>();
    let answer2 = solve_part2(&parsed);

    println!("{}\n{}", answer1, answer2);
}
