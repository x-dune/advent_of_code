#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <ranges>
#include <string>
#include <utility>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using IntPair = std::pair<int, int>;
using Dig = std::pair<IntPair, int64_t>;

std::map<char, IntPair> dir_map = {
    {'U', {-1, 0}},
    {'D', {1, 0}},
    {'L', {0, -1}},
    {'R', {0, 1}},
};

std::map<char, IntPair> dir_map2 = {
    {'0', {0, 1}},
    {'1', {1, 0}},
    {'2', {0, -1}},
    {'3', {-1, 0}},
};

std::vector<Dig> parse_input(std::vector<std::string> input, bool part2) {
  std::vector<Dig> result;

  for (auto line : input) {
    auto split = util::resplit(line);
    if (part2) {
      int64_t mag = std::stol("0x" + split[2].substr(2, 5), nullptr, 16);
      auto dir = dir_map2[split[2].substr(7, 1)[0]];
      result.push_back({dir, mag});
    } else {
      auto dir = dir_map[split[0][0]];
      int64_t mag = std::stol(split[1]);
      result.push_back({dir, mag});
    }
  }

  return result;
}

int64_t solve(std::vector<Dig> digs) {
  std::vector<IntPair> edges = {{0, 0}};
  IntPair pos;

  for (auto& [dir, mag] : digs) {
    for (int i = 1; i <= mag; i++) {
      IntPair next = {pos.first + i * dir.first, pos.second + i * dir.second};
      edges.push_back(next);
      if (i == mag) {
        pos = next;
      }
    }
  }

  int64_t area = 0;
  for (size_t i = 0; i < (edges.size() - 1); i++) {
    int j = i + 1;
    area +=
        (edges[i].first * edges[j].second) - (edges[j].first * edges[i].second);
  }
  area = std::abs(area) / 2;
  int64_t interior_area = area - (edges.size() / 2) + 1;

  return (edges.size() - 1) + interior_area;
}
}  // namespace

void aoc::day18(std::vector<std::string> input) {
  int64_t answer1 = solve(parse_input(input, false));
  int64_t answer2 = solve(parse_input(input, true));

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
