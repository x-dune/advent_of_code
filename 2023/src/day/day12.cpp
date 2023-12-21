#include <cstdint>
#include <iostream>
#include <iterator>
#include <map>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
std::vector<std::pair<std::string, std::vector<int>>> parse_input(
    std::vector<std::string> input, int times) {
  std::regex regex(",");
  auto result =
      input | std::ranges::views::transform([&](auto e) {
        auto split = util::resplit(e);
        auto spring_config = split[0];
        auto group_raw = util::resplit(split[1], regex);
        auto group_iter = group_raw | std::ranges::views::transform(
                                          [](auto e) { return std::stoi(e); });
        std::vector<int> group = {group_iter.begin(), group_iter.end()};

        for (int i = 0; i < times - 1; i++) {
          spring_config += "?" + split[0];
          group.insert(group.end(), std::begin(group_iter),
                       std::end(group_iter));
        }
        return std::pair(spring_config, group);
      });

  return {result.begin(), result.end()};
}

// CacheState:
// 1. position in initial
// 2. # of broken spring in current continguous group
// 3. amount of broken contiguous groups done
using CacheState = std::tuple<int, int, int>;
std::map<CacheState, int64_t> cache;
int64_t get_valid_count(std::string initial, std::vector<int> group, int i = 0,
                        int current_group = 0, int group_done = 0) {
  CacheState cache_key = {i, current_group, group_done};

  if (cache.find(cache_key) != cache.end()) {
    return cache[cache_key];
  }

  if (i == int(initial.size())) {
    // base case
    if ((group_done == int(group.size()) && current_group == 0) ||
        (group_done + 1 == int(group.size()) &&
         current_group == group[group_done])) {
      return 1;
    } else {
      return 0;
    }
  }

  int64_t result = 0;
  for (auto c : {'#', '.'}) {
    if (initial[i] == '?' || initial[i] == c) {
      if (c == '#') {
        result += get_valid_count(initial, group, i + 1, current_group + 1,
                                  group_done);
      } else if (current_group == 0) {
        result +=
            get_valid_count(initial, group, i + 1, current_group, group_done);
      } else if (group_done < int(group.size()) &&
                 current_group == group[group_done]) {
        result += get_valid_count(initial, group, i + 1, 0, group_done + 1);
      }
    }
  }

  cache.insert({cache_key, result});
  return result;
}

int64_t solve(std::vector<std::pair<std::string, std::vector<int>>> parsed) {
  int64_t result = 0;
  for (auto e : parsed) {
    result += get_valid_count(e.first, e.second);
    cache.clear();
  }
  return result;
}
}  // namespace

void aoc::day12(std::vector<std::string> input) {
  int64_t answer1 = solve(parse_input(input, 1));
  int64_t answer2 = solve(parse_input(input, 5));

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
