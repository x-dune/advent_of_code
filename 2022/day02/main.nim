import std/sequtils
import std/strutils
import std/tables

proc solve*(input: string): (int, int) =
  let lines = input.strip.splitLines.mapIt(it.splitWhitespace)
  # A/X - rock, B/Y - paper, C/Z - scissors
  let winTable = {"A": "B", "B": "C", "C": "A"}.toTable
  let loseTable = {"A": "C", "B": "A", "C": "B"}.toTable
  let scoreTable = {"X": 1, "Y": 2, "Z": 3, "A": 1, "B": 2, "C": 3}.toTable

  var answer1 = 0
  var answer2 = 0
  for _, x in lines:
    # part 1
    if x[0] == "A":
      if x[1] == "X":
        answer1 += 3
      elif x[1] == "Y":
        answer1 += 6
    elif x[0] == "B":
      if x[1] == "Y":
        answer1 += 3
      elif x[1] == "Z":
        answer1 += 6
    else:
      if x[1] == "Z":
        answer1 += 3
      elif x[1] == "X":
        answer1 += 6
    answer1 += scoreTable[x[1]]

    # part 2
    var myHand = ""
    if x[1] == "X":
      myHand = loseTable[x[0]]
    elif x[1] == "Y":
      myHand = x[0]
      answer2 += 3
    else:
      myHand = winTable[x[0]]
      answer2 += 6
    answer2 += scoreTable[myHand]
  return (answer1, answer2)

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], '\n', answers[1]
