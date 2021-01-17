import unittest
from day03.day03 import get_puzzle_input, solution1, solution2


class TestDay03(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 7)
        self.assertEqual(solution2(puzzle_input), 336)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 198)
        self.assertEqual(solution2(puzzle_input), 5140884672)


if __name__ == "__main__":
    unittest.main()
