package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var input string
	for scanner.Scan() {
		input = input + scanner.Text()
	}

	answer1 := 0
	answer2 := 0
	enabled := true
	mulExtendedPattern := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)`)
	matches := mulExtendedPattern.FindAllStringSubmatch(input, -1)
	for _, match := range matches {
		if match[0] == "do()" {
			enabled = true
		} else if match[0] == "don't()" {
			enabled = false
		} else {
			num1, _ := strconv.Atoi(match[1])
			num2, _ := strconv.Atoi(match[2])
			answer1 += num1 * num2
			if enabled {
				answer2 += num1 * num2
			}
		}
	}
	fmt.Println(answer1)
	fmt.Println(answer2)
}
