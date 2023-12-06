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
      aoc::day01, aoc::day02, aoc::day03, empty_day, empty_day, aoc::day06,
  };

  days[day_index](input);

  return 0;
}