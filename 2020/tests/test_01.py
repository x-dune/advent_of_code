import unittest
from day01.day01 import get_puzzle_input, solution1, solution2


class TestDay01(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 514579)
        self.assertEqual(solution2(puzzle_input), 241861950)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 1010884)
        self.assertEqual(solution2(puzzle_input), 253928438)


if __name__ == "__main__":
    unittest.main()
