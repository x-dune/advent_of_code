from pathlib import Path


def get_puzzle_input(test_case=0):
    def parse_input(line):
        [instruc, arg] = line.split(" ")
        return (instruc, int(arg))

    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [parse_input(x) for x in f.read().splitlines()]


def run_instructions(instrucs):
    used_index = []
    acc = 0
    i = 0
    try:
        while i not in used_index:
            (instruc, arg) = instrucs[i]
            used_index.append(i)

            if instruc == "acc":
                acc += arg
            elif instruc == "jmp":
                i += arg
                continue
            i += 1
    except IndexError:
        pass
    return (acc, i == len(instrucs))


def solution1(lines):
    return run_instructions(lines)[0]


def solution2(lines):
    for i in range(len(lines)):
        (instruc, arg) = lines[i]
        if instruc == "nop" or instruc == "jmp":
            changed_lines = lines.copy()
            changed_lines[i] = ("jmp" if instruc == "nop" else "nop", arg)

            (acc, did_terminate) = run_instructions(changed_lines)
            if did_terminate:
                return acc


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
