from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return f.read().splitlines()


def solution1(lines):
    count = 0
    for i in range(1, len(lines)):
        if lines[i][(i * 3) % len(lines[i])] == "#":
            count = count + 1
    return count


def solution2(lines):
    count1 = 0
    count2 = 0
    count3 = 0
    count4 = 0
    count5 = 0
    for i in range(1, len(lines)):
        line_length = len(lines[i])
        if lines[i][i % line_length] == "#":
            count1 = count1 + 1
        if lines[i][(i * 3) % line_length] == "#":
            count2 = count2 + 1
        if lines[i][(i * 5) % line_length] == "#":
            count3 = count3 + 1
        if lines[i][(i * 7) % line_length] == "#":
            count4 = count4 + 1
        if i < len(lines) / 2 and lines[i * 2][i % line_length] == "#":
            count5 = count5 + 1
    return count1 * count2 * count3 * count4 * count5


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
