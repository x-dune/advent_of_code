import std/algorithm
import std/sequtils
import std/strutils
import std/nre
import std/sugar
import std/deques

type Quad = array[4, int] # ore, clay, obsidian
type Blueprint = array[4, Quad] # cost for ore, clay, obsidian, geode robot,
type State = (int, Quad, Quad, Quad) # minute, mined, resources, robots

proc `+`(a, b: Quad): Quad = [a[0]+b[0], a[1]+b[1], a[2]+b[2], a[3]+b[3]]
proc `-`(a, b: Quad): Quad = [a[0]-b[0], a[1]-b[1], a[2]-b[2], a[3]-b[3]]
proc `*`(a, b: Quad): Quad = [a[0]*b[0], a[1]*b[1], a[2]*b[2], a[3]*b[3]]
proc `<=`(a, b: Quad): bool = a.zip(b).allIt(it[0] <= it[1])

proc minedHeuristics(s: State): int = return
  ([2, 20, 200, 2000]*s[1]).foldl(a+b) + # mined
  ([1, 10, 100, 1000]*s[3]).foldl(a+b) # robots

proc compare(a, b: State): int =
  let minedA = a.minedHeuristics
  let minedB = b.minedHeuristics

  if minedA < minedB: return -1
  elif minedA > minedB: return 1
  else: return 0

proc bfs(blueprint: Blueprint, maxMinute: int): int =
  let start: State = (1, [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0])
  var q = [start].toDeque
  var depth = 1
  var maxGeode = 0
  while q.len > 0:
    if q.peekFirst()[0] > depth:
      depth = q.peekFirst()[0]
      # increase size of search space if answer is too low
      q = q.toSeq.sorted(compare, Descending)[0..min(3000, q.len-1)].toDeque

    let (minute, mined, resources, robots) = q.popFirst()

    if minute == maxMinute:
      if mined[3] > maxGeode: maxGeode = mined[3]
      continue

    let nextMinute = minute+1
    let nextMined = mined+robots
    let nextResources = resources+robots

    # build robots
    for i, cost in blueprint:
      if cost <= resources:
        var nextRobots = robots
        inc nextRobots[i]
        q.addLast((nextMinute, nextMined, nextResources-cost, nextRobots))

    # do nothing
    q.addLast((nextMinute, nextMined, nextResources, robots))

  return maxGeode

proc solve*(input: string): (int, int) =
  let number = re"(\d+)"
  let lines = input.strip.splitLines.mapIt(it.findAll(number).mapIt(it.parseInt))
  let blueprints = collect(for line in lines: [
      [line[1], 0, 0, 0],
      [line[2], 0, 0, 0],
      [line[3], line[4], 0, 0],
      [line[5], 0, line[6], 0],
    ])
  let answer1 = (0..blueprints.high)
    .mapIt((it+1)*bfs(blueprints[it], 24))
    .foldl(a+b)
  let answer2 = (0..min(2, blueprints.high)) # example input only has 2 blueprints
    .mapIt(bfs(blueprints[it], 32))
    .foldl(a*b)

  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
