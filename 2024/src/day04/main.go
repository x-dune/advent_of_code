package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var grid [][]rune
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		grid = append(grid, []rune(scanner.Text()))
	}

	answer1 := 0
	answer2 := 0
	for y, column := range grid {
		for x, cell := range column {
			if cell == 'X' {
				answer1 += countNeighbourXmas(grid, y, x)
			}
			if cell == 'A' && isDiagonalCrossMas(grid, y, x) {
				answer2 += 1
			}
		}

	}
	fmt.Println(answer1)
	fmt.Println(answer2)
}

const xmas = "XMAS"

func countNeighbourXmas(row [][]rune, y, x int) int {
	count := 0
	dirs := [8][2]int{{-1, 0}, {-1, 1}, {0, 1}, {1, 1}, {1, 0}, {1, -1}, {0, -1}, {-1, -1}} // N, NE, E, SE, S, SW, W, NW
	for _, dir := range dirs {
		valid := true
		for i := 1; i < len(xmas); i++ {
			dy, dx := dir[0], dir[1]
			ny, nx := y+(dy*i), x+(dx*i)
			if ny < 0 || ny >= len(row) || nx < 0 || nx >= len(row[0]) || row[ny][nx] != rune(xmas[i]) {
				valid = false
				break
			}
		}
		if valid {
			count++
		}
	}
	return count
}

func isDiagonalCrossMas(row [][]rune, y, x int) bool {
	if y < 1 || y >= len(row)-1 || x < 1 || x >= len(row[0])-1 {
		return false
	}
	tl, br, tr, bl := row[y-1][x-1], row[y+1][x+1], row[y-1][x+1], row[y+1][x-1]

	isDiagonal1Valid := (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')
	isDiagonal2Valid := (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M')
	return isDiagonal1Valid && isDiagonal2Valid
}
