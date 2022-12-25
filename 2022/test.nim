import std/os
import std/strutils
import std/unittest

import day01/main as day01
import day02/main as day02
import day03/main as day03
import day04/main as day04
import day05/main as day05
import day06/main as day06
import day07/main as day07
import day08/main as day08
import day09/main as day09
import day10/main as day10
import day11/main as day11
import day12/main as day12
import day13/main as day13
import day14/main as day14
import day15/main as day15
import day16/main as day16
import day17/main as day17
import day18/main as day18
import day19/main as day19
import day20/main as day20
import day21/main as day21
import day23/main as day23
import day24/main as day24
import day25/main as day25

const pathPrefix = currentSourcePath.parentDir

suite "advent of code 2022":
  test "day 1":
    check(day01.solve(readFile(pathPrefix & "/day01/test.txt")) == (24000, 45000))
  test "day 2":
    check(day02.solve(readFile(pathPrefix & "/day02/test.txt")) == (15, 12))
  test "day 3":
    check(day03.solve(readFile(pathPrefix & "/day03/test.txt")) == (157, 70))
  test "day 4":
    check(day04.solve(readFile(pathPrefix & "/day04/test.txt")) == (2, 4))
  test "day 5":
    check(day05.solve(readFile(pathPrefix & "/day05/test.txt")) == ("CMZ", "MCD"))
  test "day 6":
    let testInputs = readFile(pathPrefix & "/day06/tests.txt").strip.splitLines
    const expecteds = [(7, 19), (5, 23), (6, 23), (10, 29), (11, 26)]
    for i, input in testInputs:
      check(day06.solve(input) == expecteds[i])
  test "day 7":
    check(day07.solve(readFile(pathPrefix & "/day07/test.txt")) == (95437, 24933642))
  test "day 8":
    check(day08.solve(readFile(pathPrefix & "/day08/test.txt")) == (21, 8))
  test "day 9":
    check(day09.solve(readFile(pathPrefix & "/day09/test1.txt")) == (13, 1))
    check(day09.solve(readFile(pathPrefix & "/day09/test2.txt"))[1] == 36)
  test "day 10":
    const crtDisplay = @[
      "##..##..##..##..##..##..##..##..##..##..",
      "###...###...###...###...###...###...###.",
      "####....####....####....####....####....",
      "#####.....#####.....#####.....#####.....",
      "######......######......######......####",
      "#######.......#######.......#######.....",
    ]
    check(day10.solve(readFile(pathPrefix & "/day10/test.txt")) == (13140, crtDisplay))
  test "day 11":
    check(day11.solve(readFile(pathPrefix & "/day11/test.txt")) == (10605'i64, 2713310158))
  test "day 12":
    check(day12.solve(readFile(pathPrefix & "/day12/test.txt")) == (31, 29))
  test "day 13":
    check(day13.solve(readFile(pathPrefix & "/day13/test.txt")) == (13, 140))
  test "day 14":
    check(day14.solve(readFile(pathPrefix & "/day14/test.txt")) == (24, 93))
  test "day 15":
    check(day15.solve(readFile(pathPrefix & "/day15/test.txt"), true) == (26, 56000011))
  test "day 16":
    check(day16.solve(readFile(pathPrefix & "/day16/test.txt")) == (1651, 1707))
  test "day 17":
    check(day17.solve(readFile(pathPrefix & "/day17/test.txt"))[0] == 3068)
  test "day 18":
    check(day18.solve(readFile(pathPrefix & "/day18/test1.txt"))[0] == 10)
    check(day18.solve(readFile(pathPrefix & "/day18/test2.txt")) == (64, 58))
  test "day 19":
    check(day19.solve(readFile(pathPrefix & "/day19/test.txt")) == (33, 56*62))
  test "day 20":
    check(day20.solve(readFile(pathPrefix & "/day20/test.txt")) == (3, 1623178306))
  test "day 21":
    check(day21.solve(readFile(pathPrefix & "/day21/test.txt")) == (152, 301))
  # day 22 contains a non-trivial difference in the example vs actual input
  # this causes my solution to not work for the example input
  test "day 23":
    check(day23.solve(readFile(pathPrefix & "/day23/test2.txt")) == (110, 20))
  test "day 24":
    check(day24.solve(readFile(pathPrefix & "/day24/test.txt")) == (18, 54))
  test "day 25":
    check(day25.solve(readFile(pathPrefix & "/day25/test.txt")) == "2=-1=0")
