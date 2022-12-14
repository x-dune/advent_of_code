import std/algorithm
import std/sequtils
import std/strutils
import std/tables

type Grid = Table[(int, int), char]

proc draw(grid: Grid, a, b: seq[int]): Grid =
  var nextGrid = grid
  if a[0] == b[0]:
    for i in min(a[1], b[1])..max(a[1], b[1]):
      nextGrid[(a[0], i)] = '#'
  else:
    for i in min(a[0], b[0])..max(a[0], b[0]):
      nextGrid[(i, a[1])] = '#'
  return nextGrid

proc settledAmount(grid: Grid, part2: bool): int =
  let yMax = max(grid.keys.toSeq.mapIt(it[1]).concat(@[0]).sorted())
  let start = (500, 0)
  var currentSand = start
  var grid = grid
  var settled = 0

  while true:
    let (x, y) = currentSand
    if grid.getOrDefault(start, '.') == 'O':
      break
    if y == yMax + 1:
      if part2:
        grid[currentSand] = 'O'
        currentSand = start
        inc settled
      else:
        break
    elif grid.getOrDefault((x, y+1), '.') == '.':
      inc currentSand[1]
    elif grid.getOrDefault((x-1, y+1), '.') == '.':
      currentSand = (x - 1, y + 1)
    elif grid.getOrDefault((x+1, y+1), '.') == '.':
      currentSand = (x + 1, y + 1)
    else:
      grid[currentSand] = 'O'
      currentSand = start
      inc settled
  return settled

proc solve*(input: string): (int, int) =
  let input = input.splitLines.mapIt(it.split(" -> ").mapIt(it.split(",").mapIt(it.parseInt)))

  var grid = initTable[(int, int), char]()
  for _, line in input:
    for i in 0..line.len - 2:
      grid = draw(grid, line[i], line[i+1])

  return (settledAmount(grid, false), settledAmount(grid, true))

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
