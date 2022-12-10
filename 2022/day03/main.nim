import std/strutils

proc getPriority(c: char): int =
  if c.isLowerAscii:
    return int(c) - int('a') + 1
  else:
    return int(c) - int('A') + 27

proc solve*(input: string): (int, int) =
  let lines = input.strip.splitLines
  var answer1 = 0
  var answer2 = 0
  for i, line in lines:
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
      for _, x in lines[i]:
        if lines[i+1].contains(x) and lines[i+2].contains(x):
          answer2 += getPriority(x)
          break
  return (answer1, answer2)

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
