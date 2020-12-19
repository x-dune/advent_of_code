from pathlib import Path
import re
from typing import Match


def get_puzzle_input(test_case=0):
    def parse_rule(rule):
        name, constraint = rule.split(": ")
        constraint = (
            constraint[1]
            if constraint.startswith('"')
            else [[int(y) for y in x.split()] for x in constraint.split("|")]
        )
        return (int(name), constraint)

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        rules, msgs = f.read().rstrip().split("\n\n")

        rules = dict(parse_rule(x) for x in rules.split("\n"))
        return rules, msgs.split("\n")


# stole this solution
def is_valid(string, rule, rules):
    if len(string) == 0 or len(rule) == 0:
        return len(string) == 0 and len(rule) == 0

    first_token = rule[0]

    if isinstance(first_token, str):
        if first_token == string[0]:
            return is_valid(string[1:], rule[1:], rules)
    else:
        for token in rules[first_token]:
            if is_valid(string, list(token) + rule[1:], rules):
                return True
    return False


def count_valid(rules, msgs):
    total = 0
    for msg in msgs:
        if is_valid(msg, rules[0][0], rules):
            total += 1
    return total


def solution1(rules, msgs):
    return count_valid(rules, msgs)


def solution2(rules, msgs):
    new_rules = rules.copy()
    new_rules[8] = [[42], [42, 8]]
    new_rules[11] = [[42, 31], [42, 11, 31]]

    return count_valid(new_rules, msgs)


if __name__ == "__main__":
    from sys import argv

    rules, msgs = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(rules, msgs))
    print(solution2(rules, msgs))
