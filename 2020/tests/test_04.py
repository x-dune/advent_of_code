import unittest
from day04.day04 import get_puzzle_input, solution1, solution2


class TestDay04(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 2)

    def test_test_input_2(self):
        puzzle_input = get_puzzle_input(test_case=2)

        self.assertEqual(solution2(puzzle_input), 0)

    def test_test_input_3(self):
        puzzle_input = get_puzzle_input(test_case=3)

        self.assertEqual(solution2(puzzle_input), 4)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 213)
        self.assertEqual(solution2(puzzle_input), 147)


if __name__ == "__main__":
    unittest.main()
