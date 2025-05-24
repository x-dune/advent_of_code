package main

import (
	"bufio"
	"fmt"
	// "math"
	"os"
	// "slices"
	// "strconv"
	// "strings"
)

type Pos = [2]int

func main() {
	var grid [][]rune
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		grid = append(grid, []rune(scanner.Text()))
	}

	pos := make(map[rune][][2]int)
	for y, col := range grid {
		for x, cell := range col {
			if cell != '.' {
				pos[cell] = append(pos[cell], Pos{y, x})
			}
		}
	}

	set := make(map[Pos]bool)
	for _, v := range pos {
		for i, a := range v {
			for _, b := range v[i+1:] {
				for _, n := range getAntinodes(a, b, len(grid), len(grid[0])) {
					set[n] = true
				}
			}
			// fmt.Println(string(k), a, i)
		}

		// fmt.Println(k, v)
	}

	// printGrid(grid)
	fmt.Println(len(set))
}

func printGrid(grid [][]rune) {
	for _, col := range grid {
		for _, cell := range col {
			fmt.Print(string(cell))
		}
		fmt.Print("\n")
	}
}

func getAntinodes(a, b Pos, maxY, maxX int) []Pos {
	var antinodes []Pos
	d := Pos{b[0] - a[0], b[1] - a[1]}
	antiA := Pos{a[0] - d[0], a[1] - d[1]}
	antiB := Pos{b[0] + d[0], b[1] + d[1]}
	for _, pos := range []Pos{antiA, antiB} {
		if pos[0] >= 0 && pos[0] < maxY && pos[1] >= 0 && pos[1] < maxX {
			antinodes = append(antinodes, pos)
		}
	}
	return antinodes
}
