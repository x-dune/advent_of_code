#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <numeric>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
const std::regex comma(",");
const std::regex equals("=");
const std::regex dash("-");

using Lens = std::pair<std::string, int>;

int hash(std::string s) {
  int result = 0;
  for (char c : s) {
    result = ((result + c) * 17) % 256;
  }
  return result;
}

std::vector<std::vector<Lens>> hash_map(std::string s,
                                        std::vector<std::vector<Lens>> boxes) {
  if (s.find("=") != std::string::npos) {
    auto splits = util::resplit(s, equals);
    auto label = splits[0];
    auto focal_length = stoi(splits[1]);
    auto dest = hash(label);
    auto found = std::find_if(boxes[dest].begin(), boxes[dest].end(),
                              [&](auto x) { return x.first == label; });
    if (found != boxes[dest].end()) {
      boxes[dest][std::distance(boxes[dest].begin(), found)] = {label,
                                                                focal_length};
    } else {
      boxes[dest].push_back({label, focal_length});
    }
  } else {
    auto splits = util::resplit(s, dash);
    auto label = splits[0];
    auto dest = hash(label);
    auto filtered = boxes[dest] | std::ranges::views::filter(
                                      [&](auto x) { return x.first != label; });
    boxes[dest] = {filtered.begin(), filtered.end()};
  }
  return boxes;
}

int solve_part2(std::vector<std::string> input) {
  std::vector<std::vector<Lens>> boxes;
  for (int i = 0; i < 256; i++) {
    boxes.push_back({});
  }
  for (auto s : input) {
    boxes = hash_map(s, boxes);
  }

  int total = 0;
  for (size_t i = 0; i < boxes.size(); i++) {
    if (!boxes[i].empty()) {
      for (size_t j = 0; j < boxes[i].size(); j++) {
        total += (i + 1) * (j + 1) * boxes[i][j].second;
      }
    }
  }
  return total;
}
}  // namespace

void aoc::day15(std::vector<std::string> input) {
  auto parsed = util::resplit(input[0], comma);
  int64_t answer1 =
      std::accumulate(parsed.begin(), parsed.end(), 0,
                      [](auto acc, auto curr) { return acc + hash(curr); });
  int64_t answer2 = solve_part2(parsed);

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
