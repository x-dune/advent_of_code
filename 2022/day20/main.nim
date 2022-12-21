import std/sequtils
import std/strutils
import std/sugar

const decryptionKey = 811589153

proc solver(numbers: seq[int], part2: bool): int =
  let numbers = if not part2: numbers else: numbers.mapIt(it * decryptionKey)
  var pos = collect(for i in 0..numbers.high: i)
  for _ in 1..(if not part2: 1 else: 10):
    for i, n in numbers:
      if (n mod numbers.high) == 0:
        continue
      let nodeIndex = pos.find(i)

      var newIndex: int
      if n > 0:
        newIndex = (nodeIndex+n) mod numbers.high
      else:
        newIndex = (numbers.high + ((nodeIndex+n) mod numbers.high)) mod
          numbers.high

      var newPos = pos
      newPos.delete(nodeIndex..nodeIndex)
      pos = newPos[0..newIndex-1]
        .concat(@[pos[nodeIndex]].concat(newPos[newIndex..newPos.high]))

  let ring = pos.mapIt(numbers[it])
  let zeroIndex = ring.find(0)
  return [1000, 2000, 3000]
    .mapIt(ring[(zeroIndex + it) mod ring.len])
    .foldl(a+b)

proc solve*(input: string): (int, int) =
  let numbers = input
    .strip
    .splitLines
    .mapIt(it.parseInt)
  return (solver(numbers, false), solver(numbers, true))

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
