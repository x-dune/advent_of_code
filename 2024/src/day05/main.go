package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Rules = map[string]bool
type Updates = [][]string

func main() {
	var answer1, answer2 int
	rules, updates := parseInput()
	for _, pageNumbers := range updates {
		if isCorrectOrder(pageNumbers, rules) {
			num, _ := strconv.Atoi(pageNumbers[len(pageNumbers)/2])
			answer1 += num
		} else {
			pageNumberCount := make(map[string]int)
			for i := range pageNumbers {
				pageNumberCount[pageNumbers[i]] = countLeftOrder(rules, pageNumbers, i)
			}
			sort.Slice(pageNumbers, func(i, j int) bool {
				return pageNumberCount[pageNumbers[i]] > pageNumberCount[pageNumbers[j]]
			})
			num, _ := strconv.Atoi(pageNumbers[len(pageNumbers)/2])
			answer2 += num
		}
	}
	fmt.Println(answer1)
	fmt.Println(answer2)
}

// count the times a pageNumber appears as the left side of the ordering rule
func countLeftOrder(rules Rules, pageNumbers []string, i int) int {
	count := 0
	for j := range pageNumbers {
		if i != j {
			key := pageNumbers[i] + "|" + pageNumbers[j]
			if rules[key] {
				count += 1
			}
		}
	}
	return count
}

func parseInput() (Rules, Updates) {
	rules := make(Rules)
	var updates Updates
	scanner := bufio.NewScanner(os.Stdin)
	afterRules := false
	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			afterRules = true
			continue
		}
		if !afterRules {
			rules[text] = true
		} else {
			var pageNumbers []string
			for s := range strings.SplitSeq(text, ",") {
				pageNumbers = append(pageNumbers, s)
			}
			updates = append(updates, pageNumbers)
		}
	}
	return rules, updates
}

func isCorrectOrder(pageNumbers []string, rules Rules) bool {
	for i := range pageNumbers {
		for j := i + 1; j < len(pageNumbers); j++ {
			key := pageNumbers[i] + "|" + pageNumbers[j]
			if !rules[key] {
				return false
			}
		}
	}
	return true
}
