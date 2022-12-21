import std/sequtils
import std/strutils
import std/tables
import std/options

proc operate(operator: string, a, b: float): float =
  case operator:
    of "+": return a + b
    of "-": return a - b
    of "*": return a * b
    of "/": return a / b

proc reverseOperate(operator: string, a, b: float, left: bool): float =
  case operator:
    of "+": return a - b
    of "-": return if left: a + b else: b - a
    of "*": return a / b
    of "/": return if left: a * b else: b / a

proc solve*(input: string): (int, int) =
  let monkeys = input
    .strip
    .splitLines
    .mapIt(it.split(": ")).mapIt((it[0], it[1].split(' ')))

  var solved = initTable[string, float]()

  while not solved.hasKey("root"):
    for _, (id, equation) in monkeys:
      if id in solved:
        continue
      if equation.len == 1:
        solved[id] = equation[0].parseFloat
      elif solved.hasKey(equation[0]) and solved.hasKey(equation[2]):
        solved[id] = operate(equation[1], solved[equation[0]], solved[equation[2]])

  let answer1 = solved["root"]
  # part 2
  # solved table with humn dependent equation replaced with none
  var solved2 = solved.pairs.toSeq.mapIt((it[0], some(it[1]))).toTable
  solved2["humn"] = none(float)
  var q = monkeys.filterIt(it[1].anyIt(it == "humn"))
  while q.len > 0:
    let curr = q.pop()[0]
    q.add(monkeys.filterIt(it[1].anyIt(it == curr)))
    solved2[curr] = none(float)
  # reverse solve equation until humn is found
  let monkeyTable = monkeys.toTable
  let rootEquation = monkeyTable["root"]
  var target: string
  var other: string
  if solved2[rootEquation[0]].isNone:
    target = rootEquation[0]
    other = rootEquation[2]
  else:
    target = rootEquation[2]
    other = rootEquation[0]
  var answer2 = solved2[other].get
  while target != "humn":
    let left = monkeyTable[target][0]
    let operator = monkeyTable[target][1]
    let right = monkeyTable[target][2]

    if solved2[left].isNone and solved2[right].isSome:
      target = left
      answer2 = reverseOperate(operator, answer2, solved2[right].get, true)
    elif solved2[right].isNone and solved2[left].isSome:
      target = right
      answer2 = reverseOperate(operator, answer2, solved2[left].get, false)
    else:
      raise newException(AssertionDefect, "Unreachable")

  return (answer1.toInt, answer2.toInt)

if isMainModule:
  let answers = solve(stdin.readAll)
  echo answers[0], '\n', answers[1]
