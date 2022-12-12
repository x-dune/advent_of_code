import std/algorithm
import std/heapqueue
import std/sequtils
import std/strutils
import std/sugar
import std/tables

type Point = (int, int)
type Grid = seq[seq[int]]

proc parseInput(input: string): (Grid, Point, Point, seq[Point]) =
  let lines = input
    .strip
    .splitLines

  var start: Point
  var target: Point
  var otherStarts: seq[Point] = @[]
  let grid = collect(
    for y, line in lines: collect(
        for x, c in line:
          if c == 'S':
            start = (y, x)
            0
          elif c == 'E':
            target = (y, x)
            25
          else:
            let height = int(c) - int('a')
            if height == 0:
              otherStarts.add((y, x))
            height
      )
    )
  return (grid, start, target, otherStarts)

proc solve*(input: string): (int, int) =
  let (grid, start, target, otherStarts) = parseInput(input)
  # dijkstra shortest path from end to every node
  var dist = {(target): 0}.toTable
  var q = [(0, target)].toHeapQueue

  while len(q) != 0:
    let (cost, pos) = q.pop()
    let (y, x) = pos

    for (yn, xn) in [(y+1, x), (y-1, x), (y, x+1), (y, x-1)]:
      try:
        let newCost = cost + 1
        if (grid[y][x] - grid[yn][xn]) <= 1 and
        newCost < dist.getOrDefault((yn, xn), high(int)):
          dist[(yn, xn)] = newCost
          q.push((newCost, (yn, xn)))
      except IndexDefect:
        continue

  let answer1 = dist[start]
  let answer2 = otherStarts
    .mapIt(dist.getOrDefault(it, high(int)))
    .sorted(cmp[int], Ascending)[0]
  return (answer1, answer2)

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
