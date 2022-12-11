import std/algorithm
import std/sequtils
import std/strutils
import std/sugar

type Monkey = tuple
  starting: seq[int]
  operation: (string, string)
  test: int
  trueMonkey: int
  falseMonkey: int

proc parseItem(data: seq[string]): Monkey =
  let starting = data[0].split(", ").mapIt(it.parseInt)
  let operationSplit = data[1].split(' ')
  let operation = (operationSplit[^2], operationSplit[^1])
  let test = data[2].split("by ")[1].parseInt
  let trueMonkey = data[3].split("monkey ")[1].parseInt
  let falseMonkey = data[4].split("monkey ")[1].parseInt
  return (starting, operation, test, trueMonkey, falseMonkey)

proc operate(item: int, operation: (string, string)): int =
  let operand = if operation[1] == "old": item else: operation[1].parseInt
  case operation[0]:
    of "+": return item + operand
    of "-": return item - operand
    of "*": return item * operand

proc monkeyBusiness(monkeys: seq[Monkey], part2: bool): int64 =
  let round = if part2: 10_000 else: 20
  let lcm = monkeys.mapIt(it.test).foldl(a*b)
  var items = monkeys.mapIt(it.starting)
  var inspectionCounts = collect(for _ in 0..monkeys.high: 0)

  for _ in 1..round:
    for i, monkey in monkeys:
      for _, item in items[i]:
        let rawWorry = operate(item, monkey.operation)
        let worry = if part2: rawWorry mod lcm else: rawWorry div 3
        let dest = (if worry mod monkey.test == 0: monkey.trueMonkey
          else: monkey.falseMonkey)
        items[dest].add(worry)
      inspectionCounts[i] += len(items[i])
      items[i] = @[]
  return inspectionCounts.sorted(system.cmp[int], Descending)[0..1].foldl(a*b)

proc solve*(input: string): (int64, int64) =
  let monkeys = input
    .strip
    .split("\n\n")
    .mapIt(it.splitLines[1..^1].mapIt(it.split(": ")[1])).mapIt(parseItem(it))

  return (monkeyBusiness(monkeys, false), monkeyBusiness(monkeys, true))

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
