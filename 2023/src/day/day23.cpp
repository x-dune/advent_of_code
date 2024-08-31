#include <iostream>
#include <iterator>
#include <set>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

#include "../day.h"

namespace {
struct pair_hash {
  template <class T1, class T2>
  std::size_t operator()(const std::pair<T1, T2> &p) const {
    auto h1 = std::hash<T1>{}(p.first);
    auto h2 = std::hash<T2>{}(p.second);
    return h1 ^ (h2 << 1);
  }
};

using Coord = std::pair<int, int>;
using Path = std::set<Coord>;
using Q = std::vector<std::pair<Path, Coord>>;
using Grid = std::vector<std::string>;
using Graph =
    std::unordered_map<Coord, std::vector<std::tuple<size_t, size_t, uint>>,
                       pair_hash>;

std::pair<Coord, char> const OFFSETS[4] = {
    {{-1, 0}, 'v'},
    {{1, 0}, '^'},
    {{0, -1}, '>'},
    {{0, 1}, '<'},
};

std::vector<Coord> get_neighbours(Coord &coord, Grid &grid, bool part2) {
  auto &[y, x] = coord;
  std::vector<Coord> coords;

  for (size_t i = 0; i < 4; i++) {
    auto &[dy, dx] = OFFSETS[i].first;
    int next_y = y + dy;
    int next_x = x + dx;

    if (next_y >= 0 && next_y < int(grid.size()) && next_x >= 0 &&
        next_x < int(grid[0].size())) {
      if (grid[next_y][next_x] != '#' &&
          (part2 || grid[next_y][next_x] != OFFSETS[i].second)) {
        coords.push_back({next_y, next_x});
      }
    }
  }

  return coords;
}

size_t solve1(Grid &grid) {
  std::unordered_map<Coord, int, pair_hash> dist;
  Path initialPath = {{0, 0}};
  Coord initialPos = {0, 1};
  Q q = {std::pair(initialPath, initialPos)};

  size_t max_dist = 0;

  while (q.size() > 0) {
    auto &[path, pos] = q[q.size() - 1];
    size_t current_path = 0;
    if (dist.find(pos) != dist.end()) {
      current_path = dist[pos];
    }

    if (path.size() < current_path) {
      continue;
    }

    if (pos.first == int(grid.size()) - 1 &&
        pos.second == int(grid[0].size()) - 2) {
      max_dist = std::max(max_dist, path.size() - 1);
    }

    auto neighbours = get_neighbours(pos, grid, false);

    Q next_q;
    for (size_t i = 0; i < neighbours.size(); i++) {
      Coord &next = neighbours[i];
      if (!path.contains(next)) {
        int current_cost;
        if (dist.find(next) != dist.end()) {
          current_cost = dist[next];
        } else {
          current_cost = 0;
          dist[next] = 0;
        }

        int next_cost = path.size() + 1;

        if (next_cost > current_cost) {
          auto next_path = path;
          next_path.insert(next);
          next_q.push_back({next_path, next});
        }
      }
    }
    q.pop_back();
    q.reserve(q.size() + distance(next_q.begin(), next_q.end()));
    q.insert(q.end(), next_q.begin(), next_q.end());
  }

  return max_dist;
}

uint dfs(Graph &graph, std::vector<std::vector<bool>> &seen, Coord &start,
         Coord &target, uint curr_dist) {
  if (start == target) {
    return curr_dist;
  }

  uint max_dist = 0;
  for (size_t i = 0; i < graph[start].size(); i++) {
    auto &[y, x, d] = graph[start][i];
    if (!seen[y][x]) {
      seen[y][x] = true;
      Coord next_start = {y, x};
      uint dist = dfs(graph, seen, next_start, target, curr_dist + d);
      max_dist = std::max(max_dist, dist);
      seen[y][x] = false;
    }
  }
  return max_dist;
}

uint solve2(Grid grid) {
  Graph neighbours_map;
  std::vector<std::pair<size_t, size_t>> hallways;

  for (size_t y = 0; y < grid.size(); y++) {
    auto &row = grid[y];
    for (size_t x = 0; x < row.size(); x++) {
      char &c = row[x];
      if (c != '#') {
        Coord coord = {y, x};
        auto neighbours = get_neighbours(coord, grid, true);

        if (neighbours.size() == 2) {
          hallways.push_back({y, x});
        }

        std::vector<std::tuple<size_t, size_t, uint>> neighbours_with_dist;
        for (size_t i = 0; i < neighbours.size(); i++) {
          auto &[ny, nx] = neighbours[i];
          neighbours_with_dist.push_back({ny, nx, 1});
        }

        neighbours_map[{y, x}] = neighbours_with_dist;
      }
    }
  }

  for (size_t i = 0; i < hallways.size(); i++) {
    auto &pos = hallways[i];
    auto &neighbours = neighbours_map[pos];
    auto &[y1, x1, d1] = neighbours[0];
    auto &[y2, x2, d2] = neighbours[1];

    auto &n1 = neighbours_map[{y1, x1}];
    auto found = std::find_if(n1.begin(), n1.end(),
                              [&](std::tuple<size_t, size_t, uint> e) {
                                auto &[y, x, d] = e;
                                return y == pos.first && x == pos.second;
                              });
    if (found != n1.end()) {
      auto found_index = std::distance(n1.begin(), found);
      n1[found_index] = {y2, x2, d1 + d2};
    }

    auto &n2 = neighbours_map[{y2, x2}];
    found = std::find_if(n2.begin(), n2.end(),
                         [&](std::tuple<size_t, size_t, uint> e) {
                           auto &[y, x, d] = e;
                           return y == pos.first && x == pos.second;
                         });
    if (found != n2.end()) {
      auto found_index = std::distance(n2.begin(), found);
      n2[found_index] = {y1, x1, d1 + d2};
    }
    neighbours_map.erase(neighbours_map.find(pos));
  }

  Coord target = {grid.size() - 1, grid[0].size() - 2};

  std::vector<bool> inner_seen(grid[0].size(), false);
  std::vector<std::vector<bool>> seen(grid.size(), inner_seen);
  Coord start_pos = {0, 1};
  return dfs(neighbours_map, seen, start_pos, target, 0);
}

}  // namespace

void aoc::day23(std::vector<std::string> input) {
  auto answer1 = solve1(input);
  auto answer2 = solve2(input);

  std::cout << answer1 << std::endl;
  std::cout << answer2 << std::endl;
}
