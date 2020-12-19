from pathlib import Path
import re


def get_puzzle_input(test_case=0):
    def tokenize(str):
        return re.findall(r"[\(\)\+\*\-]|\d+", str)

    file_path = "input.txt" if test_case == 0 else f"test_input_{test_case}.txt"
    with open(Path(__file__).parent.absolute() / file_path, "r") as f:
        return [tokenize(x) for x in f.read().splitlines()]


def head_is_higher_precedence(op_stack, token, part2):
    head = op_stack[-1]
    if head == "(":
        return False
    else:
        precedence = {"+": 1, "*": 0 if part2 else 1}
        return precedence[head] >= precedence[token]


def shunting_yard(tokens, part2=False):
    tokens = tokens.copy()
    output_q = []
    op_stack = []
    while len(tokens) != 0:
        token = tokens[0]
        if token in ["+", "-", "*"]:
            while len(op_stack) != 0 and head_is_higher_precedence(
                op_stack, token, part2
            ):
                output_q.append(op_stack.pop())
            op_stack.append(token)
        elif token == "(":
            op_stack.append(token)
        elif token == ")":
            while op_stack[-1] != "(":
                output_q.append(op_stack.pop())
            # discard left parantheses
            op_stack.pop()
        else:
            output_q.append(token)
        tokens.pop(0)
    while len(op_stack) != 0:
        output_q.append(op_stack.pop())
    return output_q


def postfix_solve(pf_q):
    num_stack = []
    while len(pf_q) != 0:
        if pf_q[0] in ["+", "-", "*"]:
            n2 = num_stack.pop()
            n1 = num_stack.pop()
            num_stack.append(eval(str(n1) + pf_q[0] + str(n2)))
        else:
            num_stack.append(pf_q[0])
        pf_q.pop(0)
    return num_stack[0]


def solution1(lines):
    return sum([postfix_solve(shunting_yard(x)) for x in lines])


def solution2(lines):
    return sum([postfix_solve(shunting_yard(x, True)) for x in lines])


if __name__ == "__main__":
    from sys import argv

    puzzle_input = get_puzzle_input(
        argv[argv.index("--test") + 1] if "--test" in argv[1:] else 0
    )
    print(solution1(puzzle_input))
    print(solution2(puzzle_input))
