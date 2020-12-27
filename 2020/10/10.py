from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        lines = [int(x) for x in f.read().splitlines()]
        lines.sort()
        lines.insert(0, 0)
        lines.append(lines[-1] + 3)
        return lines


def solution1(lines):
    diff = {1: 0, 2: 0, 3: 0}
    for i in range(len(lines) - 1):
        current_item = lines[i]
        next_item = lines[i + 1]
        current_diff = next_item - current_item
        diff[current_diff] += 1
    return diff[1] * diff[3]


def solution2(lines):
    path_dict = dict([(x, 0) for x in lines])
    path_dict[0] = 1

    for i in range(1, len(lines)):
        current = lines[i]
        for j in range(1, 4):
            if current - j in lines:
                path_dict[current] += path_dict[current - j]

    return path_dict[lines[-1]]


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
