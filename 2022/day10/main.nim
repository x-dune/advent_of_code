import std/sequtils
import std/strutils
import std/sugar

proc solve*(input: string): (int, seq[string]) =
  let instructions = input.strip.splitLines
  let temp = collect:
    for _, instruction in instructions:
      # prepend addx instruction with noop to simulate addx taking two cycles
      if instruction.startsWith("addx"): @["noop", instruction]
      else: @[instruction]
  # flatten seq[seq[string]] to seq[string]
  let prependedInput = temp.concat.concat

  var x = 1
  var answer1 = 0
  var answer2 = collect(for _ in 0..5: "........................................")
  for i, instruction in prependedInput:
    let cycle = i + 1
    if cycle in [20, 60, 100, 140, 180, 220]:
      answer1 += cycle * x

    let spritePositions = [x-1, x, x+1]
    let crtY = i div 40
    let crtX = i mod 40
    if crtX in spritePositions:
      answer2[crtY][crtX] = '#'

    if instruction.startsWith("addx"):
      x += instruction.splitWhitespace[1].parseInt
  return (answer1, answer2)

if isMainModule:
  let answers = solve(readAll(stdin))
  echo answers[0], "\n"
  for _, line in answers[1]:
    echo line
