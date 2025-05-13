package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	var input [][]int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		nums_raw := strings.Split(scanner.Text(), " ")
		nums_raw[0] = strings.TrimSuffix(nums_raw[0], ":")
		var nums []int
		for _, num_raw := range nums_raw {
			num, _ := strconv.Atoi(num_raw)
			nums = append(nums, num)
		}
		input = append(input, nums)
	}

	answer1 := 0
	answer2 := 0
	for _, nums := range input {
		answer1 += check(nums[0], nums[1:], false)
		answer2 += check(nums[0], nums[1:], true)
	}
	fmt.Println(answer1)
	fmt.Println(answer2)
}

func check(testValue int, nums []int, part2 bool) int {
	q := []int{nums[0]}
	for _, r := range nums[1:] {
		if len(q) == 0 {
			// return early, all intermediate values are higher than testValue
			return 0
		}
		var nextQ []int
		for _, l := range q {
			if l <= testValue {
				nextQ = append(nextQ, l+r)
				nextQ = append(nextQ, l*r)

				if part2 {
					nextQ = append(nextQ, concatenate(l, r))
				}
			}
		}
		q = nextQ
	}

	if slices.Contains(q, testValue) {
		return testValue
	} else {
		return 0
	}
}

func concatenate(n1, n2 int) int {
	digits := len(strconv.Itoa(n2))
	return int(math.Pow10(digits))*n1 + n2
}
