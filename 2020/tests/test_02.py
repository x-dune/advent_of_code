import unittest
from day02.day02 import get_puzzle_input, solution1, solution2


class TestDay02(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 2)
        self.assertEqual(solution2(puzzle_input), 1)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 580)
        self.assertEqual(solution2(puzzle_input), 611)


if __name__ == "__main__":
    unittest.main()
