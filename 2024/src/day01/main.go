package main

import (
	"2024/src/utils"
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

func main() {
	var left, right []int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		var leftNum, rightNum int
		fmt.Sscanf(fields[0], "%d", &leftNum)
		fmt.Sscanf(fields[1], "%d", &rightNum)
		left = append(left, leftNum)
		right = append(right, rightNum)
	}

	sort.Ints(left)
	sort.Ints(right)

	var answer1 = 0
	var answer2 = 0
	var count = make(map[int]int)
	for i := 0; i < len(left); i++ {
		diff := utils.Abs(left[i] - right[i])
		count[right[i]]++
		answer1 += diff
	}
	for _, num := range left {
		if val, exists := count[num]; exists {
			answer2 += num * val
		}
	}

	fmt.Printf("%d\n%d\n", answer1, answer2)
}
