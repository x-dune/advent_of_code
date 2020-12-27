from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [list(x) for x in f.read().splitlines()]


def expand_pocket(pocket):
    size = len(pocket[0][0]) + 2
    return (
        [[[["."] * size] * size] * (len(pocket) + 2)]
        + [expand_cube(x) for x in pocket]
        + [[[["."] * size] * size] * (len(pocket) + 2)]
    )


def expand_cube(cube):
    def expand_grid(grid):
        size = len(grid) + 2
        return [["."] * size] + [["."] + x + ["."] for x in grid] + [["."] * size]

    grid_size = len(cube[0]) + 2
    return (
        [[["."] * grid_size] * grid_size]
        + [expand_grid(x) for x in cube]
        + [[["."] * grid_size] * grid_size]
    )


def get_active_count(slices, curr_x, curr_y, curr_z):
    active_count = 0
    for x in range(curr_x - 1, curr_x + 2):
        for y in range(curr_y - 1, curr_y + 2):
            for z in range(curr_z - 1, curr_z + 2):
                try:
                    if x == -1 or y == -1 or z == -1:
                        continue
                    if not (x == curr_x and z == curr_z and y == curr_y):
                        if slices[z][y][x] == "#":
                            active_count += 1
                except IndexError:
                    pass
    return active_count


def get_active_count2(pocket, curr_x, curr_y, curr_z, curr_w):
    active_count = 0
    for x in range(curr_x - 1, curr_x + 2):
        for y in range(curr_y - 1, curr_y + 2):
            for z in range(curr_z - 1, curr_z + 2):
                for w in range(curr_w - 1, curr_w + 2):
                    try:
                        if x == -1 or y == -1 or z == -1 or w == -1:
                            continue
                        if not (
                            x == curr_x and z == curr_z and y == curr_y and w == curr_w
                        ):
                            if pocket[w][z][y][x] == "#":
                                active_count += 1
                    except IndexError:
                        pass
    return active_count


def total_active(x):
    if not isinstance(x, list):
        return 1 if x == "#" else 0
    else:
        return sum([total_active(y) for y in x])


def solution1(puzzle_input):
    cube = [puzzle_input]
    for _n in range(6):
        next_cube = []
        cube = expand_cube(cube)
        for z in range(len(cube)):
            next_grid = []
            for y in range(len(cube[z])):
                next_row = []
                for x in range(len(cube[z][y])):
                    # check neighbour
                    active_count = get_active_count(cube, x, y, z)
                    current = cube[z][y][x]
                    if current == "#":
                        if active_count == 2 or active_count == 3:
                            next_row.append("#")
                        else:
                            next_row.append(".")
                    else:
                        if active_count == 3:
                            next_row.append("#")
                        else:
                            next_row.append(".")
                next_grid.append(next_row)
            next_cube.append(next_grid)
        cube = next_cube
    return total_active(cube)


def solution2(puzzle_input):
    pocket = [[puzzle_input]]
    for _n in range(6):
        next_pocket = []
        pocket = expand_pocket(pocket)
        for w in range(len(pocket)):
            next_cube = []
            for z in range(len(pocket[w])):
                next_grid = []
                for y in range(len(pocket[w][z])):
                    next_row = []
                    for x in range(len(pocket[w][z][y])):
                        # check neighbour
                        active_count = get_active_count2(pocket, x, y, z, w)
                        current = pocket[w][z][y][x]
                        if current == "#":
                            if active_count == 2 or active_count == 3:
                                next_row.append("#")
                            else:
                                next_row.append(".")
                        else:
                            if active_count == 3:
                                next_row.append("#")
                            else:
                                next_row.append(".")
                    next_grid.append(next_row)
                next_cube.append(next_grid)
            next_pocket.append(next_cube)
        pocket = next_pocket
    return total_active(pocket)


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
