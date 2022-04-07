use std::collections::BTreeMap;

fn parse_literal(bin: &str) -> (u64, &str) {
    let mut parsed_bin = String::new();

    for i in 0.. {
        let current = &bin[(5 * i)..(5 * (i + 1))];

        parsed_bin.push_str(&current[1..]);
        if current.starts_with('0') {
            let value = u64::from_str_radix(&parsed_bin, 2).unwrap();
            let leftover = &bin[(5 * (i + 1))..];
            return (value, leftover);
        }
    }
    unreachable!();
}

fn operate(type_id: u64, sub_packets: Vec<u64>) -> u64 {
    match type_id {
        0 => sub_packets.iter().sum(),
        1 => sub_packets.iter().product(),
        2 => *sub_packets.iter().min().unwrap(),
        3 => *sub_packets.iter().max().unwrap(),
        5 => (sub_packets[0] > sub_packets[1]) as u64,
        6 => (sub_packets[0] < sub_packets[1]) as u64,
        7 => (sub_packets[0] == sub_packets[1]) as u64,
        _ => unreachable!(),
    }
}

fn parse(bin: &str) -> (u64, u64, &str) {
    let version = u64::from_str_radix(&bin[..3], 2).unwrap();
    let type_id = u64::from_str_radix(&bin[3..6], 2).unwrap();

    if type_id == 4 {
        // literal value
        let (value, leftover) = parse_literal(&bin[6..]);
        (version, value, leftover)
    } else {
        // operator packet
        let i = bin.chars().nth(6).unwrap().to_digit(2).unwrap();

        if i == 0 {
            // total length of subpackets
            let l = usize::from_str_radix(&bin[7..22], 2).unwrap();
            let mut subp_version_sum = 0;
            let mut subp_values = vec![];
            let mut remainder = &bin[22..22 + l];

            while !remainder.is_empty() {
                let (next_subp_version_sum, next_subp_value, next_remainder) = parse(remainder);

                subp_version_sum += next_subp_version_sum;
                remainder = next_remainder;
                subp_values.push(next_subp_value);
            }

            return (
                version + subp_version_sum,
                operate(type_id, subp_values),
                &bin[22 + l..],
            );
        } else if i == 1 {
            // number of subpackets
            let l = usize::from_str_radix(&bin[7..18], 2).unwrap();

            let mut subp_version_sum = 0;
            let mut subp_values = vec![];
            let mut remainder = &bin[18..];
            for _ in 0..l {
                let (next_subp_version_sum, next_subp_value, next_remainder) = parse(remainder);
                subp_version_sum += next_subp_version_sum;
                remainder = next_remainder;
                subp_values.push(next_subp_value);
            }

            return (
                version + subp_version_sum,
                operate(type_id, subp_values),
                remainder,
            );
        }
        unreachable!()
    }
}

fn main() {
    let hex = include_str!("input.txt").trim();
    let hex_to_bin = BTreeMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    let bin = hex.chars().map(|c| hex_to_bin[&c]).collect::<String>();
    let (version_sum, value, _) = parse(&bin);
    println!("{}\n{}", version_sum, value);
}
