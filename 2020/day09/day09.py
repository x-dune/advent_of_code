from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [int(x) for x in f.read().splitlines()]


def solution1(lines, is_test):
    preamble = 5 if is_test else 25

    for i in range(preamble, len(lines)):
        target = lines[i]

        try:
            for j in range(i - preamble, i - 1):
                for k in range(j + 1, i):
                    if j == i - 2 and k == i - 1:
                        return target
                    elif lines[j] + lines[k] == target:
                        # break nested loop
                        raise Exception
        except Exception:
            pass


def solution2(lines, solution1_answer):
    target = solution1_answer

    for i in range(0, len(lines) - 1):
        for j in range(1, len(lines)):
            contiguous = lines[i:j]
            contiguous_sum = sum(contiguous)

            if contiguous_sum == target:
                return min(contiguous) + max(contiguous)
            elif contiguous_sum > target:
                break


if __name__ == "__main__":
    from sys import argv

    is_test = True if "--test" in argv[1:] else False
    puzzle_input = get_puzzle_input(argv[argv.index("--test") + 1] if is_test else 0)
    solution1_answer = solution1(puzzle_input, is_test)
    print(solution1_answer)
    print(solution2(puzzle_input, solution1_answer))
