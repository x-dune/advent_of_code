from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return list(map(int, (f.read().rstrip())))


def get_next_cups(cups, part2=False):
    n = 1_000_000 if part2 else 9
    next_cups = [None for _ in range(n + 1)]

    for i in range(len(cups) - 1):
        next_cups[cups[i]] = cups[i + 1]

    if not part2:
        next_cups[cups[-1]] = cups[0]
    else:
        next_cups[cups[-1]] = 10
        for i in range(10, 1_000_000):
            next_cups[i] = i + 1
        next_cups[1_000_000] = cups[0]

    return next_cups


def shuffle_cups(puzzle_input, part2=False):
    next_cups = get_next_cups(puzzle_input, part2)
    current = puzzle_input[0]
    max_cup = len(next_cups) - 1
    for _ in range(10_000_000 if part2 else 100):
        pick_up1 = next_cups[current]
        pick_up2 = next_cups[pick_up1]
        pick_up3 = next_cups[pick_up2]

        pick_ups = (pick_up1, pick_up2, pick_up3)

        dest = max_cup if current == 1 else current - 1
        while dest in pick_ups:
            dest = max_cup if dest == 1 else dest - 1

        next_cups[current] = next_cups[pick_up3]
        next_cups[pick_up3] = next_cups[dest]
        next_cups[dest] = pick_up1

        current = next_cups[current]

    return next_cups


def solution1(puzzle_input):
    next_cups = shuffle_cups(puzzle_input)

    result = []
    current = next_cups[1]
    while current != 1:
        result.append(current)
        current = next_cups[current]
    return "".join(map(str, result))


def solution2(puzzle_input):
    next_cups = shuffle_cups(puzzle_input, part2=True)

    next_to_1 = next_cups[1]
    next_next_to_1 = next_cups[next_to_1]

    return next_to_1 * next_next_to_1


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
