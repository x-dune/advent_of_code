from pathlib import Path
from collections import Counter, defaultdict, deque
from time import time
import re


def get_puzzle_input(test_case=0):
    dir_re = r"e|se|sw|w|nw|ne"

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [re.findall(dir_re, x) for x in f.read().splitlines()]


def step(coord, dir):
    x, y, z = coord
    if dir == "e":
        return (x + 1, y - 1, z)
    if dir == "ne":
        return (x + 1, y, z - 1)
    if dir == "se":
        return (x, y - 1, z + 1)
    if dir == "w":
        return (x - 1, y + 1, z)
    if dir == "nw":
        return (x, y + 1, z - 1)
    if dir == "sw":
        return (x - 1, y, z + 1)


def get_neighbours(coord):
    return [
        step(coord, "e"),
        step(coord, "ne"),
        step(coord, "se"),
        step(coord, "w"),
        step(coord, "nw"),
        step(coord, "sw"),
    ]


def get_initial_black_tiles(lines):
    coords = []
    current = 0, 0, 0
    for line in lines:
        for dir in line:
            current = step(current, dir)
        coords.append(current)
        current = 0, 0, 0

    return {coord: count for coord, count in Counter(coords).items() if count % 2 == 1}


def solution1(initial_black_tiles):
    return len(initial_black_tiles)


def solution2(initial_black_tiles):
    black_tiles = initial_black_tiles
    for _ in range(100):
        coords = []
        for coord in black_tiles:
            coords.extend(get_neighbours(coord))
        black_tiles = {
            coord: count
            for coord, count in Counter(coords).items()
            if count == 2 or (count == 1 and coord in black_tiles)
        }
    return len(black_tiles)


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    initial_black_tiles = get_initial_black_tiles(puzzle_input)
    print(solution1(initial_black_tiles))
    print(solution2(initial_black_tiles))
