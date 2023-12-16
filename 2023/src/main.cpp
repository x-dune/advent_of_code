#include <functional>
#include <iostream>
#include <string>
#include <vector>

#include "day.h"

void empty_day(std::vector<std::string> input) {}

int main(int argc, char *argv[]) {
  std::vector<std::string> input;
  std::string temp;

  while (std::getline(std::cin, temp)) {
    input.push_back(temp);
  }

  int day_index = std::stoi(argv[1]) - 1;

  std::vector<std::function<void(std::vector<std::string>)>> days = {
      aoc::day01, aoc::day02, aoc::day03, aoc::day04, aoc::day05, aoc::day06,
      aoc::day07, aoc::day08, aoc::day09, aoc::day10, aoc::day11, aoc::day12,
      aoc::day13, aoc::day14, aoc::day15, aoc::day16};

  days[day_index](input);

  return 0;
}