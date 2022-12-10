import std/algorithm
import std/sequtils
import std/strutils

let input = readAll(stdin)
  .strip
  .split("\n\n").mapIt(it.splitLines.mapIt(parseInt(it)))

var caloriesSum = input.mapIt(it.foldl(a + b))
  .sorted(system.cmp[int], Descending)
let answer1 = caloriesSum[0]
let answer2 = caloriesSum[0] + caloriesSum[1] + caloriesSum[2]

echo answer1, '\n', answer2
