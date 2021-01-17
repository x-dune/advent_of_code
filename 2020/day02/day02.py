from pathlib import Path


def get_puzzle_input(test_case=0):
    def parseInputLine(line):
        [num1_and_num2, rawLetter, password] = line.split(" ")
        [num1, num2] = [int(x) for x in num1_and_num2.split("-")]
        return (num1, num2, rawLetter[0], password)

    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [parseInputLine(x) for x in f.read().splitlines()]


def solution1(lines):
    count = 0
    for (num1, num2, letter, password) in lines:
        occurence = password.count(letter)
        if occurence >= num1 and occurence <= num2:
            count = count + 1
    return count


def solution2(lines):
    count = 0
    for (num1, num2, letter, password) in lines:
        # XOR
        if (password[num1 - 1] == letter) != (password[num2 - 1] == letter):
            count = count + 1
    return count


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
