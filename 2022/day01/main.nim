import std/algorithm
import std/sequtils
import std/strutils

proc solve*(input: string): (int, int) =
  let calorieGroups = input
    .strip
    .split("\n\n").mapIt(it.splitLines.mapIt(parseInt(it)))

  var caloriesSum = calorieGroups.mapIt(it.foldl(a + b))
    .sorted(system.cmp[int], Descending)

  return (caloriesSum[0], caloriesSum[0] + caloriesSum[1] + caloriesSum[2])

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
