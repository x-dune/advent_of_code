import unittest
from day08.day08 import get_puzzle_input, solution1, solution2


class TestDay08(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 5)
        self.assertEqual(solution2(puzzle_input), 8)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 1420)
        self.assertEqual(solution2(puzzle_input), 1245)


if __name__ == "__main__":
    unittest.main()
