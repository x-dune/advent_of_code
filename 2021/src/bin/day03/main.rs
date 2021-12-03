use aoc2021::util;

fn parsed_input() -> Vec<Vec<char>> {
    return util::input_as_lines(include_str!("input.txt"))
        .iter()
        .map(|x| x.chars().collect())
        .collect();
}

fn solution1() -> isize {
    let input = parsed_input();

    let mut count_of_1_bit: Vec<i32> = vec![0; input[0].len()];
    let mut i = 0;
    while i < input.len() {
        let mut j = 0;
        while j < input[i].len() {
            if input[i][j] == '1' {
                count_of_1_bit[j] += 1;
            }
            j += 1;
        }

        i += 1;
    }

    let mut k = 0;
    let mut gamma_binary = "".to_owned();
    let mut epsilon_binary = "".to_owned();
    while k < count_of_1_bit.len() {
        if count_of_1_bit[k] > input.len() as i32 / 2 {
            gamma_binary.push_str("1");
            epsilon_binary.push_str("0");
        } else {
            gamma_binary.push_str("0");
            epsilon_binary.push_str("1");
        }
        k += 1;
    }

    return isize::from_str_radix(&gamma_binary, 2).unwrap()
        * isize::from_str_radix(&epsilon_binary, 2).unwrap();
}

fn solution2() -> isize {
    let mut input1 = parsed_input();

    let mut bit_position = 0;
    while input1.len() > 1 {
        let mut count_of_1_bit = 0;
        let mut i = 0;
        while i < input1.len() {
            if input1[i][bit_position] == '1' {
                count_of_1_bit += 1;
            }
            i += 1;
        }

        if count_of_1_bit as f64 >= input1.len() as f64 / 2.0 {
            input1 = input1
                .into_iter()
                .filter(|x| x[bit_position] == '1')
                .collect()
        } else {
            input1 = input1
                .into_iter()
                .filter(|x| x[bit_position] == '0')
                .collect()
        }
        bit_position += 1;
    }

    let mut input2 = parsed_input();

    let mut bit_position = 0;
    while input2.len() > 1 {
        let mut count_of_1_bit = 0;
        let mut i = 0;
        while i < input2.len() {
            if input2[i][bit_position] == '1' {
                count_of_1_bit += 1;
            }
            i += 1;
        }

        if count_of_1_bit as f64 >= input2.len() as f64 / 2.0 {
            input2 = input2
                .into_iter()
                .filter(|x| x[bit_position] == '0')
                .collect()
        } else {
            input2 = input2
                .into_iter()
                .filter(|x| x[bit_position] == '1')
                .collect()
        }
        bit_position += 1;
    }
    let bin1 = input1[0].iter().collect::<String>();
    let bin2 = input2[0].iter().collect::<String>();

    return isize::from_str_radix(&bin1, 2).unwrap() * isize::from_str_radix(&bin2, 2).unwrap();
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
