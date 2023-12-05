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
}  // namespace util