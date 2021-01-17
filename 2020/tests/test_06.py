import unittest
from day06.day06 import get_puzzle_input, solution1, solution2


class TestDay06(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 11)
        self.assertEqual(solution2(puzzle_input), 6)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 6778)
        self.assertEqual(solution2(puzzle_input), 3406)


if __name__ == "__main__":
    unittest.main()
