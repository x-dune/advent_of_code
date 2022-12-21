import std/math
import std/sequtils
import std/strutils
import std/tables

var solved = initTable[string, float64]()
proc solver(table: Table[string, seq[string]], target: string, testIntegerDivision: bool=false): float64 =
  if solved.hasKey(target):
    return solved[target]
  else:
    let equation = table[target]
    case equation[1]:
      of "+":
        solved[target] = solver(table, equation[0],) + solver(table, equation[2])
      of "-":
        solved[target] = solver(table, equation[0]) - solver(table, equation[2])
      of "*":
        solved[target] = solver(table, equation[0]) * solver(table, equation[2])
      of "/":
        solved[target] = solver(table, equation[0]) / solver(table, equation[2])
    return solved[target]

proc solve*(input: string): (float64, float64) =
  var input = input.strip.splitLines.mapIt(it.split(": ")).mapIt((it[0], it[1].split(' '))).toTable

  for k,v in input.pairs:
    if v.len == 1:
      solved[k] = v[0].parseFloat
  let tempSolved = solved.deepCopy

  echo solver(input, "root").toInt

  # binary search that doens't work on the test input
  # todo Fix
  var max = 100_000_000_000_000.0
  var min = -100_000_000_000_000.0
  while true:
    let guess = ((max + min)/2).floor
    solved = tempSolved.deepCopy
    solved["humn"] = guess
    let diff = solver(input, input["root"][2]) - solver(input, input["root"][0])
    if diff < 0:
      min = guess
    elif diff > 0:
      max = guess
    else:
      echo guess.toInt
      break
  
if isMainModule:
  discard solve(stdin.readAll)
  # let answers = solve(stdin.readAll)
  # echo answers[0], '\n', answers[1]