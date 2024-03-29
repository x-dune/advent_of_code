import std/algorithm
import std/sequtils
import std/strutils
import std/tables

proc solve*(input: string): (string, string) =
  let parts = input.split("\n\n").mapIt(it.splitLines)

  let stackAmount = ((len(parts[0][0]) - 3) div 4) + 1
  var crates = initTable[int, seq[char]]()

  for i, line in parts[0].reversed:
    for j in 1..stackAmount:
      if i == 0:
        crates[parseInt($line[1 + (j-1) * 4])] = @[]
        continue
      let c = line[1 + (j-1) * 4]
      if c != ' ':
        crates[j].add(c)

  var cratesPart2 = crates

  for _, line in parts[1]:
    let instructions = line.splitWhitespace
    let amount = parseInt($instructions[1])
    let source = parseInt($instructions[3])
    let target = parseInt($instructions[5])
    # part 1
    for i in 1..amount:
      crates[target].add(crates[source][^1])
      crates[source].delete(len(crates[source]))
    # part 2
    let startIndex = len(cratesPart2[source]) - amount
    cratesPart2[target].add(cratesPart2[source][startIndex..^1])
    cratesPart2[source].delete(startIndex..len(cratesPart2[source]) - 1)

  var answer1 = ""
  var answer2 = ""
  for i in 1..stackAmount:
    answer1 &= crates[i][^1]
    answer2 &= cratesPart2[i][^1]
  return (answer1, answer2)


if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
