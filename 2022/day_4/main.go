package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
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

	numRangesContainTheOther := 0
	for scanner.Scan() {

		sections := strings.Split(scanner.Text(), ",")

		elfOneIds := strings.Split(sections[0], "-")
		elfTwoIds := strings.Split(sections[1], "-")

		elfOneStart, err := strconv.Atoi(elfOneIds[0])
		elfOneEnd, err := strconv.Atoi(elfOneIds[1])

		elfTwoStart, err := strconv.Atoi(elfTwoIds[0])
		elfTwoEnd, err := strconv.Atoi(elfTwoIds[1])

		if err != nil {
			fmt.Printf("there was an issue converting from strings to ints...")
			break
		}

		elfOneRange := makeRange(elfOneStart, elfOneEnd)
		elfTwoRange := makeRange(elfTwoStart, elfTwoEnd)

		if contains(elfOneRange, elfTwoStart) && contains(elfOneRange, elfTwoEnd) {
			// first range contains the second
			numRangesContainTheOther++
			continue
		}

		if contains(elfTwoRange, elfOneStart) && contains(elfTwoRange, elfOneEnd) {
			// first range contains the second
			numRangesContainTheOther++
			continue
		}
	}

	fmt.Printf("part one: %d\n", numRangesContainTheOther)

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

	numRangesOverlap := 0
	for scanner.Scan() {

		sections := strings.Split(scanner.Text(), ",")

		elfOneIds := strings.Split(sections[0], "-")
		elfTwoIds := strings.Split(sections[1], "-")

		elfOneStart, err := strconv.Atoi(elfOneIds[0])
		elfOneEnd, err := strconv.Atoi(elfOneIds[1])

		elfTwoStart, err := strconv.Atoi(elfTwoIds[0])
		elfTwoEnd, err := strconv.Atoi(elfTwoIds[1])

		if err != nil {
			fmt.Printf("there was an issue converting from strings to ints...")
			break
		}

		elfOneRange := makeRange(elfOneStart, elfOneEnd)
		elfTwoRange := makeRange(elfTwoStart, elfTwoEnd)

		for _, v := range elfOneRange {
			if contains(elfTwoRange, v) {
				numRangesOverlap++
				break
			}
		}
	}

	fmt.Printf("part two: %d\n", numRangesOverlap)
}

func contains(s []int, v int) bool {
	for _, val := range s {
		if val == v {
			return true
		}
	}
	return false
}

func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}
