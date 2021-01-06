from pathlib import Path
from itertools import combinations


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [int(x) for x in f.read().splitlines()]


def solution1(lines):
    for (x, y) in combinations(lines, 2):
        if x + y == 2020:
            return x * y


def solution2(lines):
    for (x, y, z) in combinations(lines, 3):
        if x + y + z == 2020:
            return x * y * z


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
