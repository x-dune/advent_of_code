use aoc2021::util;

type Board = Vec<Vec<i32>>;
type Boards = Vec<Board>;

const MARKED_VALUE: i32 = -1;

fn get_drawn_numbers() -> Vec<i32> {
    let input = util::input_as_string(include_str!("input.txt"));
    let input_splitted = input.split("\n\n").collect::<Vec<&str>>();
    let drawn_numbers = input_splitted[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    drawn_numbers
}

fn get_boards() -> Boards {
    let input = util::input_as_string(include_str!("input.txt"));
    let boards = input
        .split("\n\n")
        .skip(1)
        .map(|a| {
            a.split('\n')
                .map(|b| {
                    b.split_whitespace()
                        .map(|z| z.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect::<Boards>();

    boards
}

fn is_board_a_winner(board: &Board) -> bool {
    for row in board {
        if row.iter().all(|x| *x == MARKED_VALUE) {
            return true;
        }
    }

    let transposed = util::transpose(board.to_vec());
    for row in transposed {
        if row.iter().all(|x| *x == MARKED_VALUE) {
            return true;
        }
    }
    false
}

fn mark_board(board: &mut Board, number: i32) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            if cell == &number {
                *cell = MARKED_VALUE;
            }
        }
    }
}

fn sum_board(board: &Board) -> i32 {
    let mut sum = 0;
    for row in board {
        for cell in row {
            if *cell != MARKED_VALUE {
                sum += cell;
            }
        }
    }
    sum
}

fn solution1() -> i32 {
    let drawn_numbers = get_drawn_numbers();
    let mut boards = get_boards();
    for number in drawn_numbers {
        for board in boards.iter_mut() {
            mark_board(board, number);
            if is_board_a_winner(board) {
                return sum_board(board) * number;
            }
        }
    }
    0
}

fn solution2() -> i32 {
    let drawn_numbers = get_drawn_numbers();
    let mut boards = get_boards();
    let boards_length = boards.len();

    let mut winner_indexes: Vec<usize> = vec![];
    for number in drawn_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if !winner_indexes.contains(&i) {
                mark_board(board, number);
                if is_board_a_winner(board) {
                    winner_indexes.push(i);
                    if boards_length == winner_indexes.len() {
                        return sum_board(board) * number;
                    }
                }
            }
        }
    }
    0
}

fn main() {
    println!("{} {}", solution1(), solution2())
}
