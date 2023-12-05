#include <functional>
#include <iostream>
#include <string>
#include <vector>

#include "day.h"

int main(int argc, char *argv[]) {
  std::vector<std::string> input;
  std::string temp;

  while (std::getline(std::cin, temp)) {
    input.push_back(temp);
  }

  int day_index = std::stoi(argv[1]) - 1;

  std::vector<std::function<void(std::vector<std::string>)>> days = {
      aoc::day01,
      aoc::day02,
  };

  days[day_index](input);

  return 0;
}