import unittest
from day09.day09 import get_puzzle_input, solution1, solution2


class TestDay09(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)
        answer1 = solution1(puzzle_input, is_test=True)
        self.assertEqual(answer1, 127)
        self.assertEqual(solution2(puzzle_input, answer1), 62)

    def test_input(self):
        puzzle_input = get_puzzle_input()
        answer1 = solution1(puzzle_input, is_test=False)
        self.assertEqual(answer1, 90433990)
        self.assertEqual(solution2(puzzle_input, answer1), 11691646)


if __name__ == "__main__":
    unittest.main()
