#include <algorithm>
#include <deque>
#include <iostream>
#include <optional>
#include <random>
#include <regex>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "../day.h"
#include "../util.h"

namespace {

using namespace std;

using Graph = unordered_map<string, unordered_set<string>>;

Graph parse_input(const vector<string>& input) {
  Graph graph;
  for (const auto& line : input) {
    size_t colon_pos = line.find(": ");
    string component = line.substr(0, colon_pos);
    auto neighbours = util::resplit(line.substr(colon_pos + 2), regex(" "));
    for (const auto& neighbour : neighbours) {
      graph[component].insert(neighbour);
      graph[neighbour].insert(component);
    }
  }
  return graph;
}

vector<string> bfs(const Graph& graph, const string& start,
                   const string& target) {
  deque<pair<string, vector<string>>> q = {{start, {start}}};
  unordered_set<string> seen = {start};

  while (!q.empty()) {
    auto [current, path] = q.front();
    q.pop_front();
    for (const auto& neighbour : graph.at(current)) {
      if (!seen.contains(neighbour)) {
        vector<string> next_path = path;
        next_path.push_back(neighbour);
        if (neighbour == target) {
          return next_path;
        }
        seen.insert(neighbour);
        q.push_back({neighbour, next_path});
      }
    }
  }
  return {};  // Return an empty vector if no path is found
}

size_t component_size(const Graph& graph, const string& start) {
  unordered_set<string> seen;
  deque<string> q = {start};
  while (!q.empty()) {
    string current = q.front();
    q.pop_front();
    if (seen.contains(current)) {
      continue;
    }
    seen.insert(current);
    if (graph.find(current) != graph.end()) {
      for (const auto& neighbour : graph.at(current)) {
        q.push_back(neighbour);
      }
    }
  }
  return seen.size();
}

optional<size_t> solve(const Graph& graph) {
  unordered_map<string, size_t> count;
  vector<string> keys;
  for (const auto& [key, _] : graph) {
    keys.push_back(key);
  }

  random_device rd;
  mt19937 rng(rd());
  uniform_int_distribution<> dist(0, graph.size() - 1);

  // pick two random nodes and find the path between them
  // keep count of all the nodes in the path
  // repeat until arbitrarily confident
  for (int i = 0; i < 1000; ++i) {
    const string& a = keys[dist(rng)];
    const string& b = keys[dist(rng)];
    auto path = bfs(graph, a, b);
    if (!path.empty()) {
      for (const auto& node : path) {
        if (node != a && node != b) {
          count[node]++;
        }
      }
    }
  }

  vector<pair<string, size_t>> sorted(count.begin(), count.end());
  sort(sorted.begin(), sorted.end(),
       [](const auto& a, const auto& b) { return a.second > b.second; });

  Graph cut_graph = graph;
  // remove the six most commonly occuring nodes
  // which should be the most connected nodes
  for (size_t i = 0; i < 6 && i < sorted.size(); ++i) {
    cut_graph.erase(sorted[i].first);
  }

  // pick two random nodes and count the size of their connected cluster
  // assuming the clusters are of different sizes
  // iterate until the nodes are in different clusters and thus have different
  // sizes
  for (int i = 0; i < 1000; ++i) {
    const string& a = keys[dist(rng)];
    const string& b = keys[dist(rng)];

    size_t size1 = component_size(cut_graph, a);
    size_t size2 = component_size(cut_graph, b);

    if (size1 != size2) {
      return size1 * size2;
    }
  }

  // unlikely to happen, try increasing iterations for finding the two clusters
  return nullopt;
}
}  // namespace

void aoc::day25(std::vector<std::string> input) {
  auto parsed = parse_input(input);
  auto answer1 = solve(parsed);

  std::cout << answer1.value() << std::endl;
}
