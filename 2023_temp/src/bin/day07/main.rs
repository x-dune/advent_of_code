use std::{
    cmp::Ordering,
    collections::BTreeMap,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone)]
struct Hand {
    chars: Vec<char>,
    bid: i32,
    hand_type: HandType,
    hand_type_2: HandType,
}

fn get_hand_type(card_counts: &[(char, i32)], part2: bool) -> HandType {
    let mut card_counts = Vec::from(card_counts);
    let mut hand_type = HandType::HighCard;
    let joker_count = if part2 {
        let count = card_counts
            .iter()
            .find_map(|(x, y)| if x == &'J' { Some(*y) } else { None })
            .unwrap_or(0);

        if count > 0 {
            card_counts = card_counts
                .into_iter()
                .filter(|(x, _)| x != &'J')
                .collect::<Vec<_>>();
        }

        count
    } else {
        0
    };

    if joker_count == 5 || card_counts[0].1 + joker_count == 5 {
        hand_type = HandType::FiveOfAKind;
    } else if card_counts[0].1 + joker_count == 4 {
        hand_type = HandType::FourOfAKind;
    } else if card_counts[0].1 + joker_count == 3 {
        if card_counts[1].1 == 2 {
            hand_type = HandType::FullHouse;
        } else {
            hand_type = HandType::ThreeOfAKind;
        }
    } else if card_counts[0].1 + joker_count == 2 {
        if card_counts[1].1 == 2 {
            hand_type = HandType::TwoPair;
        } else {
            hand_type = HandType::OnePair;
        }
    }
    hand_type
}

fn parse_input(input: Vec<String>) -> Vec<Hand> {
    input
        .iter()
        .map(|x| {
            let config = x.split_once(' ').unwrap();
            let cards = config.0.chars().collect::<Vec<_>>();
            let bid = config.1.parse::<i32>().unwrap();

            let mut card_counts: BTreeMap<char, i32> = BTreeMap::new();

            for card in cards.clone() {
                card_counts.entry(card).or_insert(0);
                let entry = card_counts.get_mut(&card).unwrap();
                *entry += 1;
            }

            let mut card_counts = card_counts.into_iter().collect::<Vec<_>>();
            card_counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            let hand_type = get_hand_type(&card_counts, false);
            let hand_type_2 = get_hand_type(&card_counts, true);

            Hand {
                chars: cards,
                bid,
                hand_type,
                hand_type_2,
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let card_order = BTreeMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    let mut card_order2 = card_order.clone();
    *card_order2.get_mut(&'J').unwrap() = 0;

    let mut hands = parse_input(input);
    let mut hands2 = hands.clone();

    hands.sort_by(|a, b| {
        let comp = b.hand_type.partial_cmp(&a.hand_type).unwrap();

        if comp == Ordering::Equal {
            let mut comp2 = comp;
            for i in 0..5 {
                comp2 = card_order
                    .get(&a.chars[i])
                    .unwrap()
                    .partial_cmp(card_order.get(&b.chars[i]).unwrap())
                    .unwrap();
                if comp2 != Ordering::Equal {
                    break;
                }
            }
            comp2
        } else {
            comp
        }
    });

    hands2.sort_by(|a, b| {
        let comp = b.hand_type_2.partial_cmp(&a.hand_type_2).unwrap();

        if comp == Ordering::Equal {
            let mut comp2 = Ordering::Equal;
            for i in 0..5 {
                comp2 = card_order2
                    .get(&a.chars[i])
                    .unwrap()
                    .partial_cmp(card_order2.get(&b.chars[i]).unwrap())
                    .unwrap();
                if comp2 != Ordering::Equal {
                    break;
                }
            }
            comp2
        } else {
            comp
        }
    });

    let answer1 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as i32 + 1) * hand.bid);
    let answer2 = hands2
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as i32 + 1) * hand.bid);

    println!("{}\n{}", answer1, answer2);
}
