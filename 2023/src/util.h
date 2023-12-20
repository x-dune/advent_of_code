#pragma once

#include <regex>
#include <string>
#include <vector>

namespace util {
std::vector<std::string> resplit(const std::string &string,
                                 const std::regex &pattern = std::regex{
                                     "\\s+"});

std::string join(const std::vector<std::string> &strings,
                 const std::string separator);

int64_t lcm(const std::vector<int64_t> ns);
}  // namespace util