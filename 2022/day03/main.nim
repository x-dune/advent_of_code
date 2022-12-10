import std/strutils

let input = readAll(stdin).strip.splitLines

proc getPriority(c: char): int =
  if c.isLowerAscii:
    return int(c) - int('a') + 1
  else:
    return int(c) - int('A') + 27

var answer1 = 0
var answer2 = 0
for i, line in input:
  # part 1
  let halfLength = len(line) div 2
  let compartment1 = line[0..halfLength-1]
  let compartment2 = line[halfLength..^1]
  for _, x in compartment1:
    if compartment2.contains(x):
      answer1 += getPriority(x)
      break
  # part 2
  if i mod 3 == 0:
    for _, x in input[i]:
      if input[i+1].contains(x) and input[i+2].contains(x):
        answer2 += getPriority(x)
        break

echo answer1, '\n', answer2
