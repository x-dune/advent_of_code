#include <algorithm>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <ranges>
#include <regex>
#include <string>
#include <tuple>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using Map = std::map<std::string, std::pair<std::string, std::string>>;

std::pair<Map, std::vector<std::string>> parse_input(
    std::vector<std::string> input) {
  std::string directions = input[0];
  Map map;
  std::vector<std::string> ghost_start;
  auto regex1 = std::regex(" = ");
  auto regex2 = std::regex(", ");

  for (int i = 2; i < input.size(); i++) {
    auto split1 = util::resplit(input[i], regex1);
    auto split2 =
        util::resplit(split1[1].substr(1, split1[1].size() - 2), regex2);
    if (split1[0].ends_with('A')) {
      ghost_start.push_back(split1[0]);
    }
    map.insert({split1[0], std::pair(split2[0], split2[1])});
  }

  return std::pair(map, ghost_start);
}

int navigate(std::string start, Map map, std::string directions,
             bool (*predicate)(std::string)) {
  int steps = 0;
  auto current = start;

  while (predicate(current)) {
    auto i = steps % directions.size();
    auto direction = directions[i];
    if (direction == 'L') {
      current = map[current].first;
    } else {
      current = map[current].second;
    }
    steps++;
  }

  return steps;
}

std::vector<int> get_prime_factors(int number) {
  int n = number;
  std::vector<int> result;

  while (n % 2 == 0) {
    result.push_back(n);
    n /= 2;
  }

  int i = 3;
  while (i <= std::floor(std::sqrt(n))) {
    i += 2;
    while (n % i == 0) {
      result.push_back(i);
      n /= i;
    }
  }

  if (n > 2) {
    result.push_back(n);
  }

  return result;
}

int64_t get_lcm(std::vector<int> ns) {
  auto prime_factors = ns | std::ranges::views::transform(get_prime_factors);
  std::map<int, int> max_count_of_factor;

  for (auto factors : prime_factors) {
    std::map<int, int> count_of_factor;

    for (auto factor : factors) {
      if (count_of_factor.find(factor) == count_of_factor.end()) {
        count_of_factor.insert({factor, 1});
      } else {
        count_of_factor[factor] += 1;
      };
    }

    for (auto entry : count_of_factor) {
      if (max_count_of_factor.find(entry.first) == max_count_of_factor.end()) {
        max_count_of_factor.insert(entry);
      } else if (entry.second > max_count_of_factor[entry.first]) {
        max_count_of_factor[entry.first] = entry.second;
      };
    }
  }

  int64_t lcm = 1;
  for (auto x : max_count_of_factor) {
    lcm *= std::pow(x.first, x.second);
  }

  return lcm;
}

}  // namespace

void aoc::day08(std::vector<std::string> input) {
  auto parsed = parse_input(input);
  auto directions = input[0];

  int answer1 = navigate("AAA", parsed.first, directions,
                         [](auto x) { return x != "ZZZ"; });
  auto ghost_results =
      parsed.second | std::ranges::views::transform([&](auto x) {
        return navigate(x, parsed.first, directions,
                        [](auto x) { return !x.ends_with('Z'); });
      });
  int64_t answer2 = get_lcm({ghost_results.begin(), ghost_results.end()});

  std::cout << answer1 << '\n' << answer2 << std::endl;
}
