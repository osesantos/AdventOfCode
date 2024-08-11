package day3

import (
	"adventofcode-2020/model"
	"fmt"
)

func expandLine(line string, times int) string {
	newLine := line
	for i := 0; i < times; i++ {
		newLine = fmt.Sprint(newLine, newLine)
	}

	return newLine
}

func getInputExpanded(input model.Input) model.Input {
	var newInput model.Input

	for _, l := range input {
		// expanding 10 times for a ratio of 1 row to 10 col
		newLine := expandLine(l, 10)

		newInput = append(newInput, newLine)
	}

	return newInput
}

func hasTreeInRow(x int, row string) bool {
	if x >= len(row) {
		return false
	} else {
		return string(row[x]) == "#"
	}
}

func Day3Part1(input model.Input) int {
	treeCounter := 0
	x := 0

	expandedInput := getInputExpanded(input)

	for i, row := range expandedInput {
		if i == 0 {
			x = x + 3
			continue
		}

		if hasTreeInRow(x, row) {
			treeCounter++
		}
		x = x + 3
	}

	return treeCounter
}

func Day3Part2(input model.Input) int {
	return 0
}
