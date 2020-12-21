from collections import defaultdict
from math import sqrt
from pathlib import Path
from operator import mul
from functools import reduce
import re


def left_edge(tile):
    return "".join([x[0] for x in tile])


def right_edge(tile):
    return "".join([x[-1] for x in tile])


def all_edges(tile):
    edges = [tile[0], tile[-1], left_edge(tile), right_edge(tile)]
    flipped_edges = [x[::-1] for x in edges]
    return edges + flipped_edges


def rotate_clockwise(tile, n):
    rotated = []
    for i in range(len(tile)):
        for j in range(len(tile)):
            point = tile[len(tile) - j - 1][i]
            try:
                rotated[i] += point
            except IndexError:
                rotated.append(point)
    if n == 1:
        return rotated
    else:
        return rotate_clockwise(rotated, n - 1)


def all_permuts(tile):
    rotated = [
        rotate_clockwise(tile, 1),
        rotate_clockwise(tile, 2),
        rotate_clockwise(tile, 3),
        rotate_clockwise(tile, 4),
    ]
    result = set(["\n".join(x) for x in rotated])
    for perm in rotated:
        result.add("\n".join(perm[::-1]))  # flip vertically
        result.add("\n".join([x[::-1] for x in perm]))  # flip horizontally
    return [x.split("\n") for x in result]


def find_right_tile(id, tile, edges_map, puzzle_input):
    r_edge = right_edge(tile)
    right_tile_id = [x for x in edges_map[r_edge] if x != id][0]
    for permut in all_permuts(puzzle_input[right_tile_id]):
        if left_edge(permut) == r_edge:
            return (right_tile_id, permut)


def find_bottom_tile(id, tile, edges_map, puzzle_input):
    bottom_edge = tile[-1]
    bottom_tile_id = [x for x in edges_map[bottom_edge] if x != id][0]
    for permut in all_permuts(puzzle_input[bottom_tile_id]):
        if permut[0] == bottom_edge:
            return (bottom_tile_id, permut)


def remove_border(tile):
    return [x[1:-1] for x in tile[1:-1]]


def get_puzzle_input(test_case=0):
    def parse_image(image):
        lines = image.split("\n")
        return (int(lines[0][4:-1]), lines[1:])  # (id, {raw_image_data, 8_edges})

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return dict(parse_image(x) for x in f.read().rstrip().split("\n\n"))


def process_puzzle_input(puzzle_input):
    edges_to_id = defaultdict(list)
    for id in puzzle_input:
        for edge in all_edges(puzzle_input[id]):
            edges_to_id[edge].append(id)

    connections = defaultdict(lambda: 0)
    for edge in edges_to_id:
        if len(edges_to_id[edge]) > 1:
            for id in edges_to_id[edge]:
                connections[id] += 0.5
    img_corners = [x for x in connections if connections[x] == 2]

    print("part1", reduce(mul, img_corners))

    # rotate until left edge and top edge is not connected
    top_left_corner = puzzle_input[img_corners[0]]
    while True:
        if (
            len(edges_to_id[left_edge(top_left_corner)]) == 1
            and len(edges_to_id[top_left_corner[0]]) == 1
        ):
            break
        else:
            top_left_corner = rotate_clockwise(top_left_corner, 1)

    img_size = int(sqrt(len(puzzle_input)))
    img = [[] for _ in range(img_size)]
    for y in range(img_size):
        for x in range(img_size):
            if x == 0 and y == 0:
                img[y].append((img_corners[0], top_left_corner))
            elif x == 0:
                top_tile = img[y - 1][0]
                next_tile = find_bottom_tile(
                    top_tile[0], top_tile[1], edges_to_id, puzzle_input
                )
                img[y].append(next_tile)
            else:
                left_tile = img[y][x - 1]
                next_tile = find_right_tile(
                    left_tile[0], left_tile[1], edges_to_id, puzzle_input
                )
                img[y].append(next_tile)

    stiched_img = []
    len_of_stiched_tile = len(remove_border(top_left_corner))
    for i in range(img_size * len_of_stiched_tile):
        current_line = "".join(
            [
                remove_border(x[1])[i % len_of_stiched_tile]
                for x in img[i // len_of_stiched_tile]
            ]
        )
        stiched_img.append(current_line)

    sea_monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]

    resea_monster = "\n".join(sea_monster).replace(" ", ".")

    sea_monster_count = 0
    len_stiched_img = len(stiched_img)
    # check each 2x30 chunk in each permutation
    for permuts in all_permuts(stiched_img):
        for y in range(len_stiched_img - len(sea_monster)):
            for x in range(len_stiched_img - len(sea_monster[0])):
                chunk = "\n".join([line[x : x + 20] for line in permuts[y : y + 3]])
                if re.match(resea_monster, chunk):
                    sea_monster_count += 1
        # if sea_monster is found, consider as correct orientation, so stop
        if sea_monster_count != 0:
            break

    total = 0
    for line in stiched_img:
        for point in line:
            if point == "#":
                total += 1

    space_occupied_by_monster = 0
    for line in sea_monster:
        for point in line:
            if point == "#":
                space_occupied_by_monster += 1
    print("part2", total - (sea_monster_count * space_occupied_by_monster))


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input()
    process_puzzle_input(puzzle_input)
