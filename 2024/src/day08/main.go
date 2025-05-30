package main

import (
	"bufio"
	"fmt"
	"os"
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

	set1 := make(map[Pos]bool)
	set2 := make(map[Pos]bool)
	for _, v := range pos {
		for i, a := range v {
			for _, b := range v[i+1:] {
				for _, n := range getAntinodes1(a, b, len(grid), len(grid[0])) {
					set1[n] = true
				}
				for _, n := range getAntinodes2(a, b, len(grid), len(grid[0])) {
					set2[n] = true
				}
			}
		}
	}

	fmt.Println(len(set1))
	fmt.Println(len(set2))
}

func isInBounds(pos Pos, maxY, maxX int) bool {
	return pos[0] >= 0 && pos[0] < maxY && pos[1] >= 0 && pos[1] < maxX
}

func getAntinodes1(a, b Pos, maxY, maxX int) []Pos {
	antinodes := []Pos{}
	d := Pos{b[0] - a[0], b[1] - a[1]}
	antiA := Pos{a[0] - d[0], a[1] - d[1]}
	antiB := Pos{b[0] + d[0], b[1] + d[1]}
	for _, pos := range []Pos{antiA, antiB} {
		if isInBounds(pos, maxY, maxX) {
			antinodes = append(antinodes, pos)
		}
	}
	return antinodes
}

func getAntinodes2(a, b Pos, maxY, maxX int) []Pos {
	// each antena that has a counterpart has an antinode on it
	antinodes := []Pos{a, b}
	d := Pos{b[0] - a[0], b[1] - a[1]}
	i := 1
	for {
		antiA := Pos{a[0] - d[0]*i, a[1] - d[1]*i}
		if isInBounds(antiA, maxY, maxX) {
			antinodes = append(antinodes, antiA)
		} else {
			break
		}
		i++
	}

	i = 0
	for {
		antiB := Pos{b[0] + d[0]*i, b[1] + d[1]*i}
		if isInBounds(antiB, maxY, maxX) {
			antinodes = append(antinodes, antiB)
		} else {
			break
		}
		i++
	}

	return antinodes
}
