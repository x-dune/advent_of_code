from pathlib import Path
import re


def get_puzzle_input(test_case=0):
    def parse_rules(rules):
        rules_dict = {}
        for rule in rules:
            (name, lower1, upper1, lower2, upper2) = re.findall(
                "([\w\s]+):\s(\d+)-(\d+) or (\d+)-(\d+)", rule
            )[0]
            rules_dict[name] = {
                "lower1": int(lower1),
                "upper1": int(upper1),
                "lower2": int(lower2),
                "upper2": int(upper2),
            }
        return rules_dict

    def parse_ticket(ticket):
        return [int(x) for x in ticket.split(",")]

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        [rules, my_ticket, nearby_tickets] = f.read().split("\n\n")
        rules = parse_rules(rules.splitlines())
        return {
            "rules": rules,
            "mine": parse_ticket(my_ticket.splitlines()[1]),
            "nearby": [parse_ticket(x) for x in nearby_tickets.splitlines()[1:]],
        }


def is_valid(n, rule):
    return (n >= rule["lower1"] and n <= rule["upper1"]) or (
        n >= rule["lower2"] and n <= rule["upper2"]
    )


def solution1(puzzle_input):
    total = 0
    occurence = 0
    for ticket in puzzle_input["nearby"]:
        for n in ticket:
            check_all = [is_valid(n, v) for v in puzzle_input["rules"].values()]
            if not any(check_all):
                occurence += 1
                total += n
    return total


def get_valid_tickets(puzzle_input):
    filtered = []
    for ticket in puzzle_input["nearby"]:
        for i, n in enumerate(ticket):
            check_all = [is_valid(n, v) for v in puzzle_input["rules"].values()]
            if not any(check_all):
                break
            elif i == len(ticket) - 1:
                filtered.append(ticket)
    return filtered


def solution2(puzzle_input):
    def recurse_remove_1(i, possible_names):
        if len(possible_names[i]) == 1:
            for k, v in possible_names.items():
                if k != i and possible_names[i][0] in v:
                    v.remove(possible_names[i][0])
                    recurse_remove_1(k, possible_names)

    tickets = get_valid_tickets(puzzle_input) + [puzzle_input["mine"]]
    possible_names = {
        i: list(puzzle_input["rules"].keys()) for i in range(len(puzzle_input["mine"]))
    }

    for i in range(len(puzzle_input["mine"])):
        for ticket in tickets:
            n = ticket[i]
            next_possible_name = []
            for name in possible_names[i]:
                if is_valid(n, puzzle_input["rules"][name]):
                    next_possible_name.append(name)
            possible_names[i] = next_possible_name
        recurse_remove_1(i, possible_names)

    total = 1
    for k, v in possible_names.items():
        if v[0].startswith("departure"):
            total *= puzzle_input["mine"][k]
    return total


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
