from functools import reduce
from operator import mul
from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        [timestamp, bus_ids] = f.read().splitlines()
        bus_ids = bus_ids.split(",")
        return (int(timestamp), [x if x == "x" else int(x) for x in bus_ids])


def solution1(lines):
    (timestamp, bus_ids) = lines
    current_timestamp = timestamp
    f_bus_ids = [x for x in bus_ids if x != "x"]
    while True:
        for bus_id in f_bus_ids:
            if current_timestamp % bus_id == 0:
                return bus_id * (current_timestamp - timestamp)
        current_timestamp += 1


def chinese_remainder_theorem(mods, rems):
    n = reduce(mul, mods)
    binixis_sum = 0
    for i in range(len(mods)):
        ni = n // mods[i]
        xi = pow(ni, -1, mods[i])
        binixis_sum += xi * rems[i] * ni
    return binixis_sum % n


def solution2(lines):
    mods_and_rems = [(x, -i) for i, x in enumerate(lines[1]) if x != "x"]
    mods, rems = zip(*mods_and_rems)
    return chinese_remainder_theorem(mods, rems)


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
