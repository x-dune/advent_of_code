#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <iterator>
#include <ranges>
#include <set>
#include <string>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using Stone = std::array<long, 6>;

std::vector<Stone> parse_input(std::vector<std::string> &input) {
  std::vector<Stone> res;
  std::regex regex1(" @ ");
  std::regex regex2(", ");
  for (size_t i = 0; i < input.size(); i++) {
    auto splits = util::resplit(input[i], regex1);
    auto pos_raw = util::resplit(splits[0], regex2);
    auto velocity_raw = util::resplit(splits[1], regex2);
    auto pos =
        pos_raw | std::ranges::views::transform([](auto x) { return stol(x); });
    auto velocity = velocity_raw | std::ranges::views::transform(
                                       [](auto x) { return stol(x); });
    res.push_back({
        {pos[0], pos[1], pos[2], velocity[0], velocity[1], velocity[2]},
    });
  }
  return res;
}

long solve1(std::vector<Stone> &input) {
  long res = 0;
  long min = 200000000000000;
  long max = 400000000000000;

  for (size_t i = 0; i < input.size() - 1; i++) {
    for (size_t j = i + 1; j < input.size(); j++) {
      auto [x1, y1, z1, vx1, vy1, vz1] = input[i];
      auto [x2, y2, z2, vx2, vy2, vz2] = input[j];
      double m1 = (double)vy1 / (double)vx1;
      double m2 = (double)vy2 / (double)vx2;
      if (m1 == m2) {
        continue;
      }

      double c1 = y1 - (m1 * x1);
      double c2 = y2 - (m2 * x2);
      double x = (c2 - c1) / (m1 - m2);
      double y = m1 * x + c1;

      if ((x < x1 && vx1 > 0) || (x > x1 && vx1 < 0) || (x < x2 && vx2 > 0) ||
          (x > x2 && vx2 < 0)) {
        continue;
      }
      if (min <= x && x <= max && min <= y && y <= max) {
        res++;
      }
    }
  }

  return res;
}

long solve2(std::vector<Stone> &input) {
  std::set<long> vx_set;
  std::set<long> vy_set;
  std::set<long> vz_set;

  for (size_t i = 0; i < input.size() - 1; i++) {
    for (size_t j = i + 1; j < input.size(); j++) {
      auto [x1, y1, z1, vx1, vy1, vz1] = input[i];
      auto [x2, y2, z2, vx2, vy2, vz2] = input[j];

      if (vx1 == vx2 && std::abs(vx1) > 100) {
        std::set<long> temp;
        auto diff = x2 - x1;
        for (int v = -1000; v < 1000; v++) {
          if (v == vx1) {
            continue;
          } else if (diff % (v - vx1) == 0) {
            temp.insert(v);
          }
        }
        if (vx_set.size() != 0) {
          std::set<long> intersect;
          std::set_intersection(vx_set.begin(), vx_set.end(), temp.begin(),
                                temp.end(),
                                std::inserter(intersect, intersect.begin()));
          vx_set = intersect;
        } else {
          vx_set = temp;
        }
      } else if (vy1 == vy2 && std::abs(vy1) > 100) {
        std::set<long> temp;
        auto diff = y2 - y1;
        for (int v = -1000; v < 1000; v++) {
          if (v == vy1) {
            continue;
          } else if (diff % (v - vy1) == 0) {
            temp.insert(v);
          }
        }
        if (vy_set.size() != 0) {
          std::set<long> intersect;
          std::set_intersection(vy_set.begin(), vy_set.end(), temp.begin(),
                                temp.end(),
                                std::inserter(intersect, intersect.begin()));
          vy_set = intersect;
        } else {
          vy_set = temp;
        }
      } else if (vz1 == vz2 && std::abs(vz1) > 100) {
        std::set<long> temp;
        auto diff = z2 - z1;
        for (int v = -1000; v < 1000; v++) {
          if (v == vz1) {
            continue;
          } else if (diff % (v - vz1) == 0) {
            temp.insert(v);
          }
        }
        if (vz_set.size() != 0) {
          std::set<long> intersect;
          std::set_intersection(vz_set.begin(), vz_set.end(), temp.begin(),
                                temp.end(),
                                std::inserter(intersect, intersect.begin()));
          vz_set = intersect;
        } else {
          vz_set = temp;
        }
      }
    }
  }

  auto vx = *next(vx_set.begin(), 0);
  auto vy = *next(vy_set.begin(), 0);
  auto vz = *next(vz_set.begin(), 0);
  auto [x1, y1, z1, vx1, vy1, vz1] = input[0];
  auto [x2, y2, z2, vx2, vy2, vz2] = input[1];

  double m1 = (double)(vy1 - vy) / (double)(vx1 - vx);
  double m2 = (double)(vy2 - vy) / (double)(vx2 - vx);
  double c1 = y1 - (m1 * x1);
  double c2 = y2 - (m2 * x2);
  long x = (c2 - c1) / (m1 - m2);
  long y = m1 * x + c1;
  long time = (x - x1) / (vx1 - vx);
  long z = z1 + (vz1 - vz) * time;

  return x + y + z;
}
}  // namespace

void aoc::day24(std::vector<std::string> input) {
  auto parsed = parse_input(input);
  auto answer1 = solve1(parsed);
  auto answer2 = solve2(parsed);
  std::cout << answer1 << std::endl;
  std::cout << answer2 << std::endl;
}
