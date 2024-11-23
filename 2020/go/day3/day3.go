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

type slope struct {
	right int
	down  int
}

func Day3Part2(input model.Input) int {
	slopes := []slope{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}

	slopeResults := []int{}
	expandedInput := getInputExpanded(input)

	for _, s := range slopes {
		treeCounter := 0
		x := 0

		for i, row := range expandedInput {
			if i%s.down != 0 {
				continue
			}

			if i == 0 {
				x = x + s.right
				continue
			}

			if hasTreeInRow(x, row) {
				treeCounter++
			}

			x = x + s.right
		}

		slopeResults = append(slopeResults, treeCounter)
	}

	return slopeResults[0] * slopeResults[1] * slopeResults[2] * slopeResults[3] * slopeResults[4]
}
