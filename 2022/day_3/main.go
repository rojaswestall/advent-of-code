package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

func main() {
	partOne()
	partTwo()
}

func partOne() {
	// open file
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// read the file line by line using scanner
	scanner := bufio.NewScanner(f)

	prioritySum := 0
	for scanner.Scan() {
		items := scanner.Text()
		firstCompartment := items[:(len(items) / 2)]
		secondCompartment := items[(len(items) / 2):]

		for _, c := range firstCompartment {
			if strings.Contains(secondCompartment, string(c)) {
				prioritySum += getItemPriority(c)
				break
			}
		}
	}
	fmt.Printf("part 1: %d\n", prioritySum)
}

func partTwo() {
	// open file
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// read the file line by line using scanner
	scanner := bufio.NewScanner(f)

	prioritySum := 0
	groupCount := 0
	firstRucksack := ""
	secondRucksack := ""
	for scanner.Scan() {
		items := scanner.Text()

		if groupCount == 0 {
			firstRucksack = items
			groupCount++
			continue
		}

		if groupCount == 1 {
			secondRucksack = items
			groupCount++
			continue
		}

		// third rucksack
		for _, c := range items {
			if strings.Contains(firstRucksack, string(c)) && strings.Contains(secondRucksack, string(c)) {
				prioritySum += getItemPriority(c)
				groupCount = 0
				break
			}
		}
	}
	fmt.Printf("part 2: %d\n", prioritySum)
}

func getItemPriority(item rune) int {
	score := 0
	if unicode.IsUpper(item) {
		score = 26
	}
	lower := strings.ToLower(string(item))
	priorities := []string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
	return score + index(priorities, lower) + 1
}

func index(s []string, val string) int {
	for i, v := range s {
		if v == val {
			return i
		}
	}
	return -1
}
