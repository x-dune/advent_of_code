package main

import (
	"2024/src/utils"
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func isValid(levels []int) bool {
	valid := true
	only_inc := true
	only_dec := true
	for i := range len(levels) - 1 {
		diff := (levels[i+1] - levels[i])
		abs_diff := utils.Abs(diff)
		if (abs_diff) < 1 || (abs_diff) > 3 {
			valid = false
			break
		}
		if diff < 0 {
			only_inc = false
		}
		if diff > 0 {
			only_dec = false
		}
	}

	return valid && (only_inc || only_dec)

}

func main() {
	var reports [][]int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		var levels []int
		for _, field := range fields {
			level, _ := strconv.Atoi(field)
			levels = append(levels, level)
		}
		reports = append(reports, levels)
	}

	answer1 := 0
	answer2 := 0
	for _, levels := range reports {
		if isValid(levels) {
			answer1++
			answer2++
			continue
		}

		for i := range len(levels) {
			var newLevels []int
			newLevels = append(newLevels, levels[0:i]...)
			newLevels = append(newLevels, levels[i+1:]...)
			if isValid(newLevels) {
				answer2++
				break
			}
		}
	}

	fmt.Printf("%d\n%d\n", answer1, answer2)
}
