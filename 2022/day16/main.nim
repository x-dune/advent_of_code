import std/algorithm
import std/deques
import std/nre
import std/sequtils
import std/strutils
import std/tables
import std/sets

type Vertex = (seq[string], int)
# minute, totalPressure, valve, opened
type State = (int, int, string, seq[string])
# minute, totalPressure, valves, opened
type State2 = (int, int, array[2, string], seq[string])
let valveRegex = re"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"

proc bfs(vertices: Table[string, Vertex]): int =
  let start: State = (1, 0, "AA", @[])
  var q = [start].toDeque
  var depth = 1
  var maxPressure = 0
  var seen = [start].toHashSet
  while q.len > 0:
    if q.peekFirst[0] > depth:
      depth = q.peekFirst[0]
      q = q.toSeq.sortedByIt(it[1]).reversed[0..min(1000, q.len-1)].toDeque

    let (minute, totalPressure, valve, opened) = q.popFirst()

    if minute == 30:
      if totalPressure > maxPressure:
        maxPressure = totalPressure
      continue

    var nextStates: seq[State]
    let (neighbours, flowRate) = vertices[valve]
    if flowRate > 0 and valve notin opened:
      nextStates.add((minute+1, totalPressure+flowRate*(30-minute), valve, opened.concat(@[valve])))

    for n in neighbours:
      nextStates.add((minute+1, totalPressure, n, opened))

    for nextState in nextStates:
      if nextState notin seen:
        seen.incl(nextState)
        q.addLast(nextState)

  return maxPressure

proc bfs2(vertices: Table[string, Vertex]): int =
  let start: State2 = (1, 0, ["AA", "AA"], @[])
  var q = [start].toDeque
  var depth = 1
  var maxPressure = 0
  var seen = [start].toHashSet
  while q.len > 0:
    if q.peekFirst[0] > depth:
      depth = q.peekFirst[0]
      q = q.toSeq.sortedByIt(it[1]).reversed[0..min(5000, q.len-1)].toDeque

    let (minute, totalPressure, valves, opened) = q.popFirst()

    if minute == 26:
      if totalPressure > maxPressure:
        maxPressure = totalPressure
      continue

    let (neighbours1, flowRate1) = vertices[valves[0]]
    var nextQ: seq[State2]
    if flowRate1 > 0 and valves[0] notin opened:
      nextQ.add((minute+1, totalPressure+flowRate1*(26-minute), valves, opened.concat(@[valves[0]])))

    for n in neighbours1:
      nextQ.add((minute+1, totalPressure, [n, valves[1]], opened))

    var nextStates: seq[State2]
    let (neighbours2, flowRate2) = vertices[valves[1]]
    for item in nextQ:
      let (minute1, totalPressure1, valves1, opened1) = item
      if flowRate2 > 0 and valves[1] notin opened1:
        nextStates.add((minute1, totalPressure1+flowRate2*(26-minute), valves1, opened1.concat(@[valves[1]])))

      for n in neighbours2:
        nextStates.add((minute1, totalPressure1, [valves1[0], n], opened1))

      for nextState in nextStates:
        if nextState notin seen:
          seen.incl(nextState)
          q.addLast(nextState)

  return maxPressure

proc solve*(input: string): (int, int) =
  let vertices: Table[string, Vertex] = input
    .strip
    .splitLines
    .mapIt(it.find(valveRegex).get.captures.toSeq.mapIt(it.get))
    .mapIt((it[0], (it[2].split(", "), it[1].parseInt))).toTable

  let answer1 = bfs(vertices)
  let answer2 = bfs2(vertices)
  return (answer1, answer2)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
