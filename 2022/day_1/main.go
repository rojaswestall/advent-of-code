package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	sortedCals := getSortedCals()
	partOne(sortedCals)
	partTwo(sortedCals)
}

func partOne(sortedCals []int) {
	fmt.Printf("part one: %v\n", sortedCals[len(sortedCals)-1])
}

func partTwo(sortedCals []int) {
	sumOfHighestThreeElves := 0
	for i := 1; i < 4; i++ {
		sumOfHighestThreeElves += sortedCals[len(sortedCals)-i]
	}
	fmt.Printf("part two: %v\n", sumOfHighestThreeElves)
}

func getSortedCals() []int {
	// open file
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// read the file line by line using scanner
	scanner := bufio.NewScanner(f)

	var highestCals []int
	currentElfCalories := 0
	for scanner.Scan() {
		cal, err := strconv.Atoi(scanner.Text())
		if err != nil {
			// empty line
			highestCals = append(highestCals, currentElfCalories)
			currentElfCalories = 0
			continue
		}
		currentElfCalories += cal
	}

	sort.Ints(highestCals)
	return highestCals
}
