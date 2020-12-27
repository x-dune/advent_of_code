from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [int(x) for x in f.read().rstrip().split(",")]


def say_number(initial_numbers, count):
    spoken = {num: i for i, num in enumerate(initial_numbers)}
    last = initial_numbers[-1]
    # offset by -1 so we don't have to do i - 1
    for i in range(len(initial_numbers) - 1, count - 1):
        next_num = i - spoken[last] if last in spoken and spoken[last] != i else 0
        spoken[last] = i
        last = next_num
    return last


def solution1(initial_numbers):
    return say_number(initial_numbers, 2020)


def solution2(initial_numbers):
    # pretty damn slow
    return say_number(initial_numbers, 30_000_000)


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
