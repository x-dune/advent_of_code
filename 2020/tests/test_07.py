import unittest
from day07.day07 import get_puzzle_input, solution1, solution2


class TestDay07(unittest.TestCase):
    def test_test_input_1(self):
        child_graph, parent_graph = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(child_graph), 4)
        self.assertEqual(solution2(parent_graph), 32)

    def test_input(self):
        child_graph, parent_graph = get_puzzle_input()

        self.assertEqual(solution1(child_graph), 268)
        self.assertEqual(solution2(parent_graph), 7867)


if __name__ == "__main__":
    unittest.main()
