#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "../day.h"

namespace {
using Grid = std::vector<std::string>;

std::vector<Grid> parse_input(std::vector<std::string> input) {
  std::vector<Grid> result;

  std::vector<std::string> temp;
  for (auto line : input) {
    if (line == "") {
      result.push_back(temp);
      temp.clear();
    } else {
      temp.push_back(line);
    }
  }
  result.push_back(temp);

  return result;
}

int get_invalid_points(std::vector<char> line, size_t i, size_t checks) {
  int invalid = 0;

  for (size_t j = 0; j < checks; j++) {
    if (line[i - j - 1] != line[i + j]) {
      invalid += 1;
    }
  }

  return invalid;
}

int get_reflection(Grid grid, int allowed_invalid) {
  // search for vertical reflection
  for (size_t i = 1; i < grid[0].size(); i++) {
    size_t checks = std::min(grid[0].size() - i, i);
    bool valid = true;
    int invalid_points = 0;
    for (auto row : grid) {
      invalid_points += get_invalid_points({row.begin(), row.end()}, i, checks);
      if (invalid_points > allowed_invalid) {
        valid = false;
        break;
      }
    }
    if (valid && invalid_points == allowed_invalid) {
      return i;
    }
  }

  // search for horizontal reflection
  for (size_t i = 1; i < grid.size(); i++) {
    size_t checks = std::min(grid.size() - i, i);
    bool valid = true;
    int invalid_points = 0;
    for (size_t j = 0; j < grid[0].size(); j++) {
      std::vector<char> column;
      for (auto line : grid) {
        column.push_back(line[j]);
      }
      invalid_points += get_invalid_points(column, i, checks);
      if (invalid_points > allowed_invalid) {
        valid = false;
        break;
      }
    }
    if (valid && invalid_points == allowed_invalid) {
      return i * 100;
    }
  }
  // failed to find reflection
  return 0;
}
}  // namespace

void aoc::day13(std::vector<std::string> input) {
  auto parsed = parse_input(input);

  int answer1 = std::accumulate(
      parsed.begin(), parsed.end(), 0,
      [](auto acc, auto x) { return acc + get_reflection(x, 0); });
  int answer2 = std::accumulate(
      parsed.begin(), parsed.end(), 0,
      [](auto acc, auto x) { return acc + get_reflection(x, 1); });

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
