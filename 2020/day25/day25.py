from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [int(x) for x in f.read().splitlines()]


def solution1(p_input):
    card_pk, door_pk = p_input

    card_loop_size = 0
    val = 1
    subject_number = 7
    while val != card_pk:
        val = (val * subject_number) % 20201227
        card_loop_size += 1

    encryption_key = 1
    for _ in range(card_loop_size):
        encryption_key = (encryption_key * door_pk) % 20201227
    return encryption_key


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
