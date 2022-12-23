import std/sequtils
import std/sets
import std/strutils
import std/tables

type Point = (int, int) # (x, y)
proc `+`(a, b: Point): Point = (a[0]+b[0], a[1]+b[1])

let adjacent = [(0, -1), (-1, -1), (1, -1), (0, 1), (-1, 1), (1, 1), (1, 0), (-1, 0)]
let checkOrder = ['N', 'S', 'W', 'E']
let checkPoints = {
  'N': [(0, -1), (-1, -1), (1, -1)], # N, NE, NW,
  'S': [(0, 1), (-1, 1), (1, 1)],    # S, SE, SW
  'W': [(-1, 0), (-1, -1), (-1, 1)], # W, NW, SW
  'E': [(1, 0), (1, -1), (1, 1)],    # E, NE, or SE
}.toTable

proc solve*(input: string): (int, int) =
  let lines = input.splitLines

  var elves = initHashSet[Point]()
  for y, line in lines:
    for x, c in line:
      if c == '#': elves.incl((x, y))

  var i = 0
  var answer1: int
  var answer2: int
  while true:
    var proposedMove = initTable[Point, seq[Point]]()
    var nextElves = initHashSet[Point]()
    var didMove = false
    for elf in elves:
      if adjacent.mapIt(it+elf).allIt(it notin elves): nextElves.incl(elf)
      else:
        for j in 0..3:
          let toCheck = checkPoints[checkOrder[(i+j) mod checkOrder.len]]
          if toCheck.mapIt(it+elf).allIt(it notin elves):
            let nextPoint = elf+toCheck[0]
            discard proposedMove.hasKeyOrPut(nextPoint, @[])
            proposedMove[nextPoint].add(elf)
            didMove = true
            break
          if j == 3: nextElves.incl(elf)

    if not didMove:
      answer2 = i+1
      break

    for (point, proposedElves) in proposedMove.pairs:
      if proposedElves.len == 1: nextElves.incl(point)
      else:
        for elf in proposedElves: nextElves.incl(elf)
    elves = nextElves

    if i == 9:
      let xs = elves.mapIt(it[0])
      let ys = elves.mapIt(it[1])
      answer1 = (max(xs)+1-min(xs))*(max(ys)+1-min(ys))-elves.len

    inc i

  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
