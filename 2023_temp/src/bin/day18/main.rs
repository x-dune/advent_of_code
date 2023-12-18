use std::io::{self, BufRead};

fn parse_input(input: &[String], part2: bool) -> Vec<(char, i64)> {
    input
        .iter()
        .map(|e| {
            let splits = e.split(' ').collect::<Vec<_>>();
            if part2 {
                let dir = match &splits[2][7..8] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    _ => 'U',
                };
                let mag = i64::from_str_radix(&splits[2][2..=6], 16).unwrap();
                (dir, mag)
            } else {
                let dir = splits[0].chars().collect::<Vec<_>>()[0];
                let mag = splits[1].parse::<i64>().unwrap();
                (dir, mag)
            }
        })
        .collect()
}

fn solve(input: &[(char, i64)]) -> i64 {
    let mut edge = vec![(0, 0)];

    let mut pos = (0, 0);
    for (dir, mag) in input {
        let offset = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            _ => (0, 1),
        };

        for n in 1..=*mag {
            let next = (pos.0 + n * offset.0, pos.1 + n * offset.1);
            edge.push(next);
            if n == *mag {
                pos = next
            }
        }
    }

    let mut area = 0;
    for i in 0..edge.len() - 1 {
        let j = i + 1;
        area += (edge[i].0 * edge[j].1) - (edge[j].0 * edge[i].1);
    }
    area = area.abs() / 2;
    let interior_area = area - (edge.len() as i64 / 2) + 1;

    (edge.len() as i64 - 1) + interior_area
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let answer1 = solve(&parse_input(&input, false));
    let answer2 = solve(&parse_input(&input, true));
    println!("{} {}", answer1, answer2);
}
