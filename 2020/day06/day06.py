from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [x.split("\n") for x in f.read().rstrip().split("\n\n")]


def count_group_yes(group):
    group_yes = {}
    for answer in group:
        for char in answer:
            try:
                group_yes[char] = group_yes[char] + 1
            except KeyError:
                group_yes[char] = 1
    return group_yes


def solution1(lines):
    return sum([len(count_group_yes(x)) for x in lines])


def solution2(lines):
    count_all_yes = 0
    for group in lines:
        group_yes = count_group_yes(group)
        for key in group_yes:
            if group_yes[key] == len(group):
                count_all_yes = count_all_yes + 1
    return count_all_yes


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
