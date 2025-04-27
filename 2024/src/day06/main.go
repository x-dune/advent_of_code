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
				break
			} else {
				guardSeen[guard] = true
			}
		} else {
			guard.yPos = ny
			guard.xPos = nx
			posSeen[[2]int{ny, nx}] = true
			// grid[ny][nx] = 'X'
		}
		// fmt.Println(guard.yPos, guard.xPos, string(guard.facing), string(grid[ny][nx]))
	}
	// for _, row := range grid {
	// 	for _, cell := range row {
	// 		fmt.Print(string(cell))
	// 	}
	// 	fmt.Print("\n")
	// }
	fmt.Println(len(posSeen))
}
