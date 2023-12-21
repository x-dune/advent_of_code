#include <algorithm>
#include <cstddef>
#include <iostream>
#include <iterator>
#include <regex>
#include <string>
#include <tuple>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using Ranges = std::vector<std::pair<int64_t, int64_t>>;

struct MappingRange {
  int64_t dest;
  int64_t src;
  int64_t length;
};

struct Mapping {
  std::string name;
  std::vector<MappingRange> mapping_ranges;
};

std::tuple<Ranges, Ranges, std::vector<Mapping>> parse_input(
    std::vector<std::string> input) {
  auto joined_input = util::join(input, "\n");
  auto input_splits = util::resplit(joined_input, std::regex("\n\n"));
  auto seed_split = util::resplit(input_splits[0]);
  Ranges seed_ranges;
  Ranges seed_ranges2;
  std::transform(seed_split.begin() + 1, seed_split.end(),
                 std::back_inserter(seed_ranges),
                 [](auto s) -> auto { return std::pair(std::stol(s), 1); });

  for (size_t i = 1; i < seed_split.size(); i += 2) {
    seed_ranges2.push_back(
        {std::stol(seed_split[i]), std::stol(seed_split[i + 1])});
  }

  std::vector<Mapping> mapping;

  for (size_t i = 1; i != input_splits.size(); i++) {
    auto map_splits = util::resplit(input_splits[i], std::regex("\n"));
    std::regex regex("\\s+");

    std::vector<MappingRange> mapping_ranges;
    for (size_t i = 1; i < map_splits.size(); i++) {
      auto map_ranges = util::resplit(map_splits[i], regex);
      mapping_ranges.push_back({std::stol(map_ranges[0]),
                                std::stol(map_ranges[1]),
                                std::stol(map_ranges[2])});
    }

    mapping.push_back(
        {.name = map_splits[0], .mapping_ranges = mapping_ranges});
  }

  return std::tuple(seed_ranges, seed_ranges2, mapping);
}

int64_t get_shortest_distance(Ranges seed_ranges,
                              std::vector<Mapping> mapping) {
  std::vector<int64_t> end_start_ranges;

  for (auto seed_range : seed_ranges) {
    Ranges current_range = {seed_range};
    Ranges next_range;

    for (size_t i = 0; i < mapping.size(); i++) {
      for (auto range : current_range) {
        int64_t start = range.first;
        int64_t length = range.second;

        while (length > 0) {
          bool mapped = false;
          int64_t shortest_dist = length;

          for (auto m : mapping[i].mapping_ranges) {
            if (start >= m.src && (m.src + m.length - 1) >= start) {
              int64_t offset = start - m.src;
              int64_t remaining_length = m.length - offset;
              int64_t range_mapped = std::min(remaining_length, length);

              length -= range_mapped;
              start += range_mapped;
              mapped = true;
              next_range.push_back(std::pair(m.dest + offset, range_mapped));
              break;
            } else if (start < m.src) {
              shortest_dist = std::min(shortest_dist, m.src - start);
            }
          }

          if (!mapped) {
            next_range.push_back(std::pair(start, shortest_dist));
            start += shortest_dist;
            length -= shortest_dist;
          }
        }
      }
      current_range = next_range;
      next_range.clear();
    }

    for (auto range : current_range) {
      end_start_ranges.push_back(range.first);
    }
  }

  return *std::min_element(end_start_ranges.begin(), end_start_ranges.end());
}
}  // namespace

void aoc::day05(std::vector<std::string> input) {
  auto parsed = parse_input(input);

  int64_t answer1 =
      get_shortest_distance(std::get<0>(parsed), std::get<2>(parsed));

  int64_t answer2 =
      get_shortest_distance(std::get<1>(parsed), std::get<2>(parsed));

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
