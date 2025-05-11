package main

import (
	"2024/src/utils"
	"bufio"
	"fmt"
	"os"
	"slices"
)

type Guard struct {
	yPos   int
	xPos   int
	facing rune
}

func main() {
	var grid [][]rune
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		grid = append(grid, []rune(scanner.Text()))
	}

	guardSequence := map[rune]rune{'^': '>', '>': 'v', 'v': '<', '<': '^'}
	guardFacings := utils.GetKey(guardSequence)
	var guard Guard
	for y, row := range grid {
		for x, cell := range row {
			if slices.Contains(guardFacings, cell) {
				guard.yPos = y
				guard.xPos = x
				guard.facing = cell
			}
		}
	}
	answer1, posSeen := travel(grid, guard, guardSequence)
	answer2 := 0
	for pos := range posSeen {
		if (pos[0] == guard.yPos && pos[1] == guard.xPos) || grid[pos[0]][pos[1]] == '#' {
			continue
		}
		grid[pos[0]][pos[1]] = '#'
		len, _ := travel(grid, guard, guardSequence)
		if len == -1 {
			answer2 += 1
		}
		// reset grid
		grid[pos[0]][pos[1]] = '.'
	}

	fmt.Println(answer1)
	fmt.Println(answer2)
}

func travel(grid [][]rune, guard Guard, guardSequence map[rune]rune) (int, map[[2]int]bool) {
	guardSeen := map[Guard]bool{guard: true}
	posSeen := map[[2]int]bool{{guard.yPos, guard.xPos}: true}
	for {
		ny := guard.yPos
		nx := guard.xPos
		switch guard.facing {
		case '^':
			ny -= 1
		case '>':
			nx += 1
		case 'v':
			ny += 1
		case '<':
			nx -= 1
		}
		if ny < 0 || ny >= len(grid) || nx < 0 || nx >= len(grid[0]) {
			break
		}
		if grid[ny][nx] == '#' {
			guard.facing = guardSequence[guard.facing]
			if guardSeen[guard] {
				// prevent infinite loops
				return -1, posSeen
			} else {
				guardSeen[guard] = true
			}
		} else {
			guard.yPos = ny
			guard.xPos = nx
			posSeen[[2]int{ny, nx}] = true
		}
	}
	return len(posSeen), posSeen
}
