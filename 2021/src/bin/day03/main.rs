use aoc2021::util;

fn parsed_input() -> Vec<Vec<char>> {
    return util::input_as_lines(include_str!("input.txt"))
        .iter()
        .map(|x| x.chars().collect())
        .collect();
}

fn string_binary_to_int(string_binary: &str) -> i32 {
    i32::from_str_radix(string_binary, 2).unwrap()
}

fn solution1() -> i32 {
    let input = parsed_input();

    let mut count_of_1_bit: Vec<i32> = vec![0; input[0].len()];
    for bits in &input {
        for (i, bit) in bits.iter().enumerate() {
            if bit == &'1' {
                count_of_1_bit[i] += 1;
            }
        }
    }

    let mut gamma_binary = "".to_owned();
    let mut epsilon_binary = "".to_owned();
    for count_of_1_bit in count_of_1_bit {
        let is_1_most_common = count_of_1_bit as f64 > input.len() as f64 / 2.0;
        if is_1_most_common {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }

    string_binary_to_int(&gamma_binary) * string_binary_to_int(&epsilon_binary)
}

enum Rating {
    OxygenGenerator,
    Co2Scrubber,
}

fn generate_rating(rating: Rating) -> String {
    let mut input = parsed_input();
    let should_filter_by_1_bit = |count_of_1_bit: f64, input_length: f64| match rating {
        Rating::OxygenGenerator => count_of_1_bit >= input_length / 2.0,
        Rating::Co2Scrubber => count_of_1_bit < input_length / 2.0,
    };

    let mut bit_position = 0;
    while input.len() > 1 {
        let mut count_of_1_bit = 0;
        for bytes in &input {
            if bytes[bit_position] == '1' {
                count_of_1_bit += 1;
            }
        }

        if should_filter_by_1_bit(count_of_1_bit as f64, input.len() as f64) {
            input = input
                .into_iter()
                .filter(|x| x[bit_position] == '1')
                .collect()
        } else {
            input = input
                .into_iter()
                .filter(|x| x[bit_position] == '0')
                .collect()
        }
        bit_position += 1;
    }

    input[0].iter().collect::<String>()
}

fn solution2() -> i32 {
    let oxygen_generator_rating = generate_rating(Rating::OxygenGenerator);
    let co2_scrubber_rating = generate_rating(Rating::Co2Scrubber);

    string_binary_to_int(&oxygen_generator_rating) * string_binary_to_int(&co2_scrubber_rating)
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
