import unittest
from day10.day10 import get_puzzle_input, solution1, solution2


class TestDay10(unittest.TestCase):
    def test_test_input_1(self):
        puzzle_input = get_puzzle_input(test_case=1)

        self.assertEqual(solution1(puzzle_input), 7 * 5)
        self.assertEqual(solution2(puzzle_input), 8)

    def test_test_input_2(self):
        puzzle_input = get_puzzle_input(test_case=2)

        self.assertEqual(solution1(puzzle_input), 22 * 10)
        self.assertEqual(solution2(puzzle_input), 19208)

    def test_input(self):
        puzzle_input = get_puzzle_input()

        self.assertEqual(solution1(puzzle_input), 2400)
        self.assertEqual(solution2(puzzle_input), 338510590509056)


if __name__ == "__main__":
    unittest.main()
