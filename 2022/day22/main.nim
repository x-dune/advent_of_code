import std/enumerate
import std/nre
import std/sequtils
import std/strutils
import std/tables

# x y
type Point = (int, int)
type Grid = Table[Point, char]

proc tryMove(point: Point, grid: Grid, magnitude, dir: int): Point =
  # right = 0, down = 1, left = 2, up = 3
  var current = point
  if dir == 0:
    for _ in 1..magnitude:
      var next = (current[0]+1, current[1])
      if not grid.hasKey(next):
        let xMin = grid.keys.toSeq.filterIt(it[1] == current[1]).mapIt(it[0]).min
        next = (xMin, current[1])

      if grid[next] == '.':
        current = next
      elif grid[next] == '#':
        break
      else:
        echo "unreachable"
  if dir == 1:
    for _ in 1..magnitude:
      var next = (current[0], current[1]+1)
      if not grid.hasKey(next):
        let yMin = grid.keys.toSeq.filterIt(it[0] == current[0]).mapIt(it[1]).min
        next = (current[0], yMin)

      if grid[next] == '.':
        current = next
      elif grid[next] == '#':
        break
      else:
        echo "unreachable"
  if dir == 2:
    for _ in 1..magnitude:
      var next = (current[0]-1, current[1])
      if not grid.hasKey(next):
        let xMax = grid.keys.toSeq.filterIt(it[1] == current[1]).mapIt(it[0]).max
        next = (xMax, current[1])

      if grid[next] == '.':
        current = next
      elif grid[next] == '#':
        break
      else:
        echo "unreachable"
  if dir == 3:
    for _ in 1..magnitude:
      var next = (current[0], current[1]-1)
      if not grid.hasKey(next):
        let yMax = grid.keys.toSeq.filterIt(it[0] == current[0]).mapIt(it[1]).max
        next = (current[0], yMax)

      if grid[next] == '.':
        current = next
      elif grid[next] == '#':
        break
      else:
        echo "unreachable"
  return current

#[
This only works for the following specific 50*50 cube arrangement, it does not work on the test input
 12
 3
45
6
]#
proc tryMove2(point: Point, grid: Grid, magnitude, dir: int): (Point, int) =
  # right = 0, down = 1, left = 2, up = 3
  var current = point
  var currentDir = dir

  for _ in 1..magnitude:
    var next: Point;
    var nextDir = currentDir;
    if currentDir == 0:
      next = (current[0]+1, current[1])
      if not grid.hasKey(next):
        if next[1] <= 50: # fell off right 2, reappear at 5 going left
          nextDir = 2
          next = (100, 150-(current[1]-1)) # y 1 -> y 150, y 50 -> y 101
        elif next[1] <= 100: # fell off right 3, reappear at 2 going up
          nextDir = 3
          next = (current[1]+50, 50) # y 51 -> x 101, y 100 -> x 150
        elif next[1] <= 150: # fell off right 5, reappear at 2 going left
          nextDir = 2
          next = (150, 150-(current[1]-1)) # y 101 -> y 50, y 150 -> y 1
        else: # fell off right 6, reappear at 5 going up
          nextDir = 3
          next = (current[1]-100, 150) # y 151 -> x 51, y 200 -> x 100
    elif currentDir == 1:
      next = (current[0], current[1]+1)
      if not grid.hasKey(next):
        if next[0] <= 50: # fell off down 6, reappear at 2 going down
          next = (current[0]+100, 1) # x 1 -> x 101, x 50 -> x 150
        elif next[0] <= 100: # fell off down 5, reappear at 6 going left
          nextDir = 2
          next = (50, current[0]+100) # x 51 -> y 151, x 100 -> x 200
        else: # fell off down 2, reappear at 3 going left
          nextDir = 2
          next = (100, current[0]-50) # x 101 -> y 51, x 150 -> y 100
    elif currentDir == 2:
      next = (current[0]-1, current[1])
      if not grid.hasKey(next):
        if next[1] <= 50: # fell off left 1, reappear at 4 going right
          nextDir = 0
          next = (1, 150-(current[1]-1)) # y 1 -> y 150, y 50 -> y 101
        elif next[1] <= 100: # fell off left 3, reappear at 4 going down
          nextDir = 1
          next = (current[1]-50, 101) # y 51 -> x 1, y 100 -> x 50
        elif next[1] <= 150: # fell off left 4, reappear at 1 going right
          nextDir = 0
          next = (51, 150-(current[1]-1)) # y 101 -> y 50, y 150 -> y 1
        else: # fell off left 6, reappear at 1 going down
          nextDir = 1
          next = (current[1]-100, 1) # y 151 -> x 51, y 200 -> x 100
    else:
      next = (current[0], current[1]-1)
      if not grid.hasKey(next):
        if next[0] <= 50: # fell off up 4, reappear at 3 going right
          nextDir = 0
          next = (51, current[0]+50) # x 1 -> y 51, x 50 -> y 100
        elif next[0] <= 100: # fell off down 1, reappear at 6 going right
          nextDir = 0
          next = (1, current[0]+100) # x 51 -> y 151, x 100 -> x 200
        else: # fell off up 2, reappear at 6 going up
          next = (current[0]-100, 200) # x 101 -> x 1, x 150 -> x 50

    if grid[next] == '.':
      current = next
      currentDir = nextDir
    elif grid[next] == '#':
      break
    else:
      echo "unreachable"
  return (current, currentDir)

proc solve(input: string): (int, int) =
  let input = input.split("\n\n")

  var grid = initTable[Point, char]()
  for y, line in enumerate(input[0].splitLines):
    for x, c in line:
      if c != ' ':
        grid[(x+1, y+1)] = c

  let r = re"(L|R)|(\d+)"
  let moves = input[1].findAll(r)

  let xMin = grid.keys.toSeq.filterIt(it[1] == 1).mapIt(it[0]).min
  # right = 0, down = 1, left = 2, up = 3
  var dir = 0
  var current = (xMin, 1)
  for move in moves:
    if move == "L":
      if dir == 0: dir = 3
      else: dir = dir - 1
    elif move == "R":
      if dir == 3: dir = 0
      else: dir = dir + 1
    else:
      let magnitude = move.parseInt
      current = tryMove(current, grid, magnitude, dir)

  let answer1 = 1000*current[1] + 4*current[0] + dir

  var dir2 = 0
  var current2 = (xMin, 1)
  for move in moves:
    if move == "L":
      if dir2 == 0: dir2 = 3
      else: dir2 = dir2 - 1
    elif move == "R":
      if dir2 == 3: dir2 = 0
      else: dir2 = dir2 + 1
    else:
      let magnitude = move.parseInt
      let moved = tryMove2(current2, grid, magnitude, dir2)
      current2 = moved[0]
      dir2 = moved[1]

  let answer2 = 1000*current2[1] + 4*current2[0] + dir2
  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
