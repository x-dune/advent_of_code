#include <algorithm>
#include <array>
#include <cstddef>
#include <iostream>
#include <iterator>
#include <regex>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {
using Coord = std::array<int, 3>;

struct State {
  std::vector<Coord> top;
  std::vector<Coord> bottom;
  std::vector<size_t> supported_by;
  bool settled;
};

using Simulation = std::map<size_t, State>;
using Coords = std::vector<std::pair<Coord, Coord>>;

const std::regex tilde("~");
const std::regex comma(",");

Coords parse_input(std::vector<std::string> input) {
  Coords parsed;

  for (auto line : input) {
    auto split1 = util::resplit(line, tilde);
    Coord coords[2];
    for (size_t i = 0; i < 2; i++) {
      auto split2 = util::resplit(split1[i], comma);

      coords[i] = {std::stoi(split2[0]), std::stoi(split2[1]),
                   std::stoi(split2[2])};
    }
    parsed.push_back({coords[0], coords[1]});
  }

  return parsed;
}

bool sort_q(std::pair<size_t, State> a, std::pair<size_t, State> b) {
  return a.second.bottom[0][2] < b.second.bottom[0][2];
}

std::vector<size_t> get_supported_by(Simulation simulation,
                                     std::vector<Coord> next_bottom) {
  std::set<Coord> next_bottom_set(next_bottom.begin(), next_bottom.end());
  std::vector<size_t> supported_by;

  for (auto curr : simulation) {
    if (curr.second.settled &&
        std::any_of(
            curr.second.top.begin(), curr.second.top.end(),
            [&](auto coord) { return next_bottom_set.contains(coord); })) {
      supported_by.push_back(curr.first);
    }
  }

  return supported_by;
}

int sum_chain_reaction(Simulation simulation,
                       std::set<size_t> cannot_disintegrate) {
  int sum = 0;
  for (auto k : cannot_disintegrate) {
    std::vector<size_t> q = {k};
    int chain_reaction =
        -1;  // start at -1 to account for initial block which is not counted
    std::set<size_t> seen;

    while (!q.empty()) {
      auto curr = q[q.size() - 1];
      q.pop_back();

      if (seen.contains(curr)) {
        continue;
      }
      seen.insert(curr);
      chain_reaction++;

      for (auto s : simulation) {
        bool other_support_exist = false;
        for (auto support : s.second.supported_by) {
          if (!seen.contains(support)) {
            other_support_exist = true;
            break;
          }
        }

        if (!other_support_exist) {
          q.push_back(s.first);
        }
      }
    }
    sum += chain_reaction;
  }
  return sum;
}

std::pair<size_t, int> simulate(Coords coords) {
  Simulation simulation;

  for (size_t i = 0; i < coords.size(); i++) {
    auto &[x1, y1, z1] = coords[i].first;
    auto &[x2, y2, z2] = coords[i].second;

    std::vector<Coord> bottom;

    for (int x = 0; x <= (x2 - x1); x++) {
      for (int y = 0; y <= (y2 - y1); y++) {
        bottom.push_back({x1 + x, y1 + y, z1});
      }
    }

    std::vector<Coord> top;

    if (z2 == z1) {
      top = bottom;
    } else {
      std::transform(bottom.begin(), bottom.end(), std::back_inserter(top),
                     [&](auto coord) {
                       Coord top_element = {coord[0], coord[1], z2};
                       return top_element;
                     });
    }

    simulation.insert({i + 1, {top, bottom, {}, false}});
  }

  std::vector<std::pair<size_t, State>> q;

  while (true) {
    if (q.empty()) {
      for (size_t i = 1; i <= simulation.size(); i++) {
        if (!simulation[i].settled) {
          q.push_back({i, simulation[i]});
        }
      }
      if (q.empty()) {
        break;
      }
      std::sort(q.begin(), q.end(), sort_q);
    }

    for (auto curr : q) {
      auto &[i, state] = curr;

      if (state.bottom[0][2] == 1) {
        simulation[i].supported_by.push_back(0);  // ground
        simulation[i].settled = true;
      } else {
        // std::vector<Coord> all_other_tops;

        // for (auto: simulation)

        std::vector<Coord> next_bottom;
        for (auto coord : state.bottom) {
          next_bottom.push_back({coord[0], coord[1], coord[2] - 1});
        }

        auto supported_by = get_supported_by(simulation, next_bottom);

        if (!supported_by.empty()) {
          simulation[i].supported_by = supported_by;
          simulation[i].settled = true;
        } else {
          std::vector<Coord> next_top;
          for (auto coord : state.top) {
            next_top.push_back({coord[0], coord[1], coord[2] - 1});
          }

          simulation[i].bottom = next_bottom;
          simulation[i].top = next_top;
        }
      }
    }
    q.clear();
  }

  std::set<size_t> cannot_disintegrate;
  for (auto &[_, v] : simulation) {
    if (v.supported_by.size() == 1 && v.supported_by[0] != 0) {
      cannot_disintegrate.insert(v.supported_by[0]);
    }
  }

  size_t answer1 = coords.size() - cannot_disintegrate.size();
  int answer2 = sum_chain_reaction(simulation, cannot_disintegrate);

  return {answer1, answer2};
}

}  // namespace

void aoc::day22(std::vector<std::string> input) {
  auto parsed = parse_input(input);
  auto answers = simulate(parsed);

  std::cout << answers.first << '\n' << answers.second << std::endl;
}
