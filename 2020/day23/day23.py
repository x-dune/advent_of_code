from pathlib import Path
from collections import deque
from time import time


class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

    def __str__(self):
        return f"{self.data}->{self.next.data if self.next else '???'}"

    def __repr__(self):
        return self.__str__()


def get_puzzle_input(test_case=0):
    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return list(map(int, (f.read().rstrip())))


def main(cups_dict, first):
    curr = cups_dict[first]
    for _ in range(10_000_000):
        pick_up = [curr.next, curr.next.next, curr.next.next.next]

        dest = 1_000_000 if curr.data == 1 else curr.data - 1
        while dest in {x.data for x in pick_up}:
            dest -= 1
            if dest == 0:
                dest = 1_000_000
        curr.next = curr.next.next.next.next
        pick_up[-1].next = cups_dict[dest].next
        cups_dict[dest].next = pick_up[0]
        curr = curr.next
    return cups_dict


def get_solution1(cups):
    found1 = 0
    answer = []
    pointer = cups.head
    while found1 != 2:
        pointer = pointer.next
        if found1 == 1 and pointer.data != 1:
            answer.append(str(pointer.data))
        if pointer.data == 1:
            found1 += 1
    return "".join(answer)


def make_cups_dict(cups):
    cups_dict = {x: Node(x) for x in range(1, 10)}
    for i in range(len(cups) - 1):
        cups_dict[cups[i]].next = cups_dict[cups[i + 1]]
    cups_dict[cups[-1]].next = cups_dict[cups[0]]
    return cups_dict


def make_cups_dict2(cups):
    n = 1_000_000
    cups_dict = {x: Node(x) for x in range(1, n + 1)}
    for i in range(len(cups) - 1):
        cups_dict[cups[i]].next = cups_dict[cups[i + 1]]

    cups_dict[cups[-1]].next = cups_dict[10]

    for i in range(10, n):
        cups_dict[i].next = cups_dict[i + 1]
    cups_dict[n].next = cups_dict[cups[0]]

    return cups_dict


def get_answer1(cups_dict):
    current = cups_dict[1]
    answer = []
    while True:
        current = current.next
        if current.data == 1:
            break
        else:
            answer.append(str(current.data))
    return "".join(answer)


if __name__ == "__main__":
    from sys import argv

    start_time = time()
    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    cups_dict2 = make_cups_dict2(puzzle_input)
    answer = main(cups_dict2, puzzle_input[0])
    print(
        answer[1].next.data,
        answer[1].next.next.data,
        answer[1].next.data * answer[1].next.next.data,
    )

    print(f"Time: {'{0:.2f}'.format(time() - start_time)} seconds")
