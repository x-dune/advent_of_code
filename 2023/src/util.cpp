#include <cmath>
#include <cstddef>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

namespace util {
std::vector<std::string> resplit(const std::string &string,
                                 const std::regex &pattern = std::regex{
                                     "\\s+"}) {
  std::sregex_token_iterator iter(string.begin(), string.end(), pattern, -1);
  std::sregex_token_iterator end;
  return {iter, end};
}
std::string join(const std::vector<std::string> &strings,
                 const std::string separator) {
  std::string result = strings[0];
  for (size_t i = 1; i < strings.size(); i++) {
    result += separator + strings[i];
  }
  return result;
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

int64_t lcm(const std::vector<int64_t> ns) {
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

}  // namespace util