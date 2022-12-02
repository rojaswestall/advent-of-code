package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Shape int

const (
	Rock Shape = iota + 1
	Paper
	Scissors
	Unknown
)

// X, Y, and Z are only shapes in part 1
func shapeFromInput(input string) Shape {
	switch input {
	case "A", "X":
		// ROCK
		return Rock
	case "B", "Y":
		// PAPER
		return Paper
	case "C", "Z":
		// SCISSORS
		return Scissors
	}
	fmt.Printf("detected unknown shape: %s.\n", input)
	return Unknown
}

func getShapeScore(s Shape) int {
	switch s {
	case Rock:
		// ROCK
		return 1
	case Paper:
		// PAPER
		return 2
	case Scissors:
		// SCISSORS
		return 3
	}
	return 0
}

func getOutcomeScore(yourShape, opponentShape Shape) int {
	// DRAW
	if opponentShape == yourShape {
		return 3
	}

	// WIN
	if (yourShape == Rock && opponentShape == Scissors) ||
		(yourShape == Paper && opponentShape == Rock) ||
		(yourShape == Scissors && opponentShape == Paper) {
		return 6
	}

	// LOSS
	return 0
}

func getShape(opponentShape Shape, outcome string) Shape {
	// DRAW
	if outcome == "Y" {
		return opponentShape
	}

	// LOSE
	if outcome == "X" {
		if opponentShape == Rock {
			return Scissors
		}
		if opponentShape == Paper {
			return Rock
		}
		return Paper
	}

	// WIN
	if opponentShape == Rock {
		return Paper
	}
	if opponentShape == Paper {
		return Scissors
	}
	return Rock
}

func main() {
	// open file
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// read the file line by line using scanner
	scanner := bufio.NewScanner(f)

	partOneScore := 0
	partTwoScore := 0
	for scanner.Scan() {
		game := scanner.Text()
		input := strings.Split(game, " ")

		partOneScore += getPartOneScore(input[0], input[1])
		partTwoScore += getPartTwoScore(input[0], input[1])
	}
	fmt.Printf("part 1: %d\n", partOneScore)
	fmt.Printf("part 2: %d\n", partTwoScore)
}

func getPartOneScore(firstInput, secondInput string) int {
	opponentShape := shapeFromInput(firstInput)
	myShape := shapeFromInput(secondInput)
	return getShapeScore(myShape) + getOutcomeScore(myShape, opponentShape)
}

func getPartTwoScore(firstInput, secondInput string) int {
	opponentShape := shapeFromInput(firstInput)
	myShape := getShape(opponentShape, secondInput)
	return getShapeScore(myShape) + getOutcomeScore(myShape, opponentShape)
}
