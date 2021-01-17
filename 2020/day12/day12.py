from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [(x[0], int(x[1:])) for x in f.read().splitlines()]


def get_degree_from_dir(dir):
    if dir == "N":
        return 0
    elif dir == "E":
        return 90
    elif dir == "S":
        return 180
    else:
        return 270


def get_dir_from_degree(degree):
    if degree == 0:
        return "N"
    elif degree == 90:
        return "E"
    elif degree == 180:
        return "S"
    else:
        return "W"


def move_action(action, value, moves):
    if action == "N":
        moves["north"] += value
    elif action == "S":
        moves["north"] -= value
    elif action == "E":
        moves["east"] += value
    elif action == "W":
        moves["east"] -= value


def rotate_waypoint(action, value, moves):
    norm_value = 360 - value if action == "L" else value
    moves_copy = moves.copy()

    if norm_value == 90:
        moves["north"] = -moves_copy["east"]
        moves["east"] = moves_copy["north"]
    elif norm_value == 180:
        moves["north"] = -moves_copy["north"]
        moves["east"] = -moves_copy["east"]
    elif norm_value == 270:
        moves["north"] = moves_copy["east"]
        moves["east"] = -moves_copy["north"]


def solution1(lines):
    dir = "E"
    moves = {"north": 0, "east": 0}

    for (action, value) in lines:
        if action == "L":
            dir = get_dir_from_degree(abs((get_degree_from_dir(dir) - value) % 360))
        elif action == "R":
            dir = get_dir_from_degree(abs((get_degree_from_dir(dir) + value) % 360))
        elif action == "F":
            move_action(dir, value, moves)
        else:
            move_action(action, value, moves)
    return abs(moves["north"]) + abs(moves["east"])


def solution2(lines):
    moves = {"north": 0, "east": 0}
    waypoint = {"north": 1, "east": 10}
    for (action, value) in lines:
        if action == "L" or action == "R":
            rotate_waypoint(action, value, waypoint)
        elif action == "F":
            moves["north"] += waypoint["north"] * value
            moves["east"] += waypoint["east"] * value
        else:
            move_action(action, value, waypoint)

    return abs(moves["north"]) + abs(moves["east"])


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
