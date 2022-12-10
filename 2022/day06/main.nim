import std/sets
import std/strutils

proc solve*(input: string): (int, int) =
  let buffer = input.strip
  var answer1 = 0
  var answer2 = 0
  var i = 0

  while answer1 == 0 or answer2 == 0:
    if answer1 == 0 and len(toHashSet(buffer[i..i+3])) == 4:
      answer1 = i + 4
    if answer2 == 0 and len(toHashSet(buffer[i..i+13])) == 14:
      answer2 = i + 14
    i += 1
  return (answer1, answer2)

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
