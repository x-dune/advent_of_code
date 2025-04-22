package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Rules = map[string]bool
type Updates = [][]string

func main() {
	answer1 := 0
	rules, updates := parseInput()
	for _, update := range updates {
		if isCorrectOrder(update, rules) {
			num, _ := strconv.Atoi(update[len(update)/2])
			answer1 += num
		}
	}
	// fmt.Println(rules)
	// fmt.Println(updates)
	fmt.Println(answer1)
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
			var update []string
			for s := range strings.SplitSeq(text, ",") {
				update = append(update, s)
			}
			updates = append(updates, update)
		}
	}
	return rules, updates
}

func isCorrectOrder(update []string, rules Rules) bool {
	for i := range update {
		for j := i + 1; j < len(update); j++ {
			key := update[i] + "|" + update[j]
			_, exists := rules[key]
			if !exists {
				fmt.Println(i, j, key)
				return false
			}
		}
	}
	return true
}
