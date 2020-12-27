from pathlib import Path
from copy import deepcopy

OCCUPIED = "#"
EMPTY = "L"
FLOOR = "."
NEIGHBOURS = (
    (-1, 0),  # N
    (1, 0),  # S
    (0, 1),  # E
    (0, -1),  # W
    (-1, 1),  # NE
    (-1, -1),  # NW
    (1, 1),  # SE
    (1, -1),  # SW
)


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [list(x) for x in f.read().splitlines()]


def count_occupied(grid, y, x):
    y_size = len(grid)
    x_size = len(grid[0])

    count = 0
    for dy, dx in NEIGHBOURS:
        yy, xx = y + dy, x + dx
        if yy >= 0 and yy < y_size and xx >= 0 and xx < x_size:
            if grid[yy][xx] == OCCUPIED:
                count += 1
    return count


def count_occupied_raycast(grid, y, x):
    y_size = len(grid)
    x_size = len(grid[0])

    count = 0
    for dy, dx in NEIGHBOURS:
        yy, xx = y + dy, x + dx

        while yy >= 0 and yy < y_size and xx >= 0 and xx < x_size:
            neighbour_cell = grid[yy][xx]
            if neighbour_cell == EMPTY:
                break
            if neighbour_cell == OCCUPIED:
                count += 1
                break
            yy, xx = yy + dy, xx + dx
    return count


def stable_occupied_count(grid, part2=False):
    current_grid = deepcopy(grid)
    count_fn = count_occupied_raycast if part2 else count_occupied
    occupied_limit = 5 if part2 else 4

    while True:
        next_grid = deepcopy(current_grid)
        for y, row in enumerate(current_grid):
            for x, cell in enumerate(row):
                if cell != FLOOR:
                    occupied_n_count = count_fn(current_grid, y, x)
                    if cell == EMPTY:
                        if occupied_n_count == 0:
                            next_grid[y][x] = OCCUPIED
                    else:
                        if occupied_n_count >= occupied_limit:
                            next_grid[y][x] = EMPTY

        if current_grid == next_grid:
            return sum(x.count(OCCUPIED) for x in current_grid)
        else:
            current_grid = next_grid


def solution1(grid):
    return stable_occupied_count(grid)


def solution2(grid):
    return stable_occupied_count(grid, part2=True)


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
