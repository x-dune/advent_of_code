import unittest
from day05.day05 import get_puzzle_input, solution1, solution2


class TestDay05(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 357)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 880)
        self.assertEqual(solution2(puzzle_input), 731)


if __name__ == "__main__":
    unittest.main()
