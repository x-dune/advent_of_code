import re
from pathlib import Path


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if not test_case else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        child_graph = {}
        parent_graph = {}

        for line in f.read().splitlines():

            parent = re.search(r"(\w+ \w+) bags contain", line).group(1)
            for count, child in re.findall(r"(\d+) (\w+ \w+) bags?", line):
                try:
                    parent_graph[parent][child] = int(count)
                except KeyError:
                    parent_graph[parent] = {child: int(count)}

                try:
                    child_graph[child].add(parent)
                except KeyError:
                    child_graph[child] = {parent}
    return (child_graph, parent_graph)


def solution1(child_graph):
    parents_set = set()

    def recurse(name):
        try:
            for parent in child_graph[name]:
                parents_set.add(parent)
                recurse(parent)
        except KeyError:
            pass

    recurse("shiny gold")
    return len(parents_set)


def solution2(parent_graph):
    def recurse(name):
        total = 0
        try:
            for bag, count in parent_graph[name].items():
                total += count + count * recurse(bag)
        except KeyError:
            pass
        return total

    return recurse("shiny gold")


if __name__ == "__main__":
    from sys import argv

    child_graph, parent_graph = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(child_graph))
    print(solution2(parent_graph))
