from pathlib import Path


def get_seat_id(seat_binary):
    min_row = 0
    max_row = 127
    for i in range(0, 7):
        if seat_binary[i] == "F":
            max_row = max_row - ((max_row - min_row + 1) // 2)
        else:
            min_row = min_row + ((max_row - min_row + 1) // 2)
    min_col = 0
    max_col = 7
    for i in range(7, 10):
        if seat_binary[i] == "L":
            max_col = max_col - ((max_col - min_col + 1) // 2)
        else:
            min_col = min_col + ((max_col - min_col + 1) // 2)
    # By the end both min_row == max_row and min_col == max_col
    # so it doens't matter if we use min or max
    return (min_row * 8) + min_col


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [get_seat_id(x) for x in f.read().splitlines()]


def solution1(lines):
    return max(lines)


def solution2(lines):
    seat_ids = lines.copy()
    seat_ids.sort()
    for i in range(0, len(seat_ids) - 1):
        if seat_ids[i] + 1 != seat_ids[i + 1]:
            return seat_ids[i] + 1


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
