from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [int(x) for x in f.read().splitlines()]


def solution1(lines):
    for i in range(0, len(lines) - 1):
        for j in range(i + 1, len(lines)):
            if lines[i] + lines[j] == 2020:
                return lines[i] * lines[j]


def solution2(lines):
    for i in range(0, len(lines) - 1):
        for j in range(i + 1, len(lines)):
            for k in range(j + 1, len(lines)):
                if lines[i] + lines[j] + lines[k] == 2020:
                    return lines[i] * lines[j] * lines[k]


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
