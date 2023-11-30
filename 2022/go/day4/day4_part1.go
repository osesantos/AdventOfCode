package day4

import (
	"main/utils"
	"strconv"
	"strings"
)

type Pair struct {
	a string
	b string
}

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	parsedLines := utils.Map(lines, func(t string) []string {
		return parseLine(t)
	})

	filteredLines := utils.Filter(parsedLines, func(i []string) bool {
		return fullyContains(i)
	})

	return strconv.Itoa(len(filteredLines))
}

func parseLine(line string) []string {
	return strings.Split(line, ",")
}

func parseSet(set string) []int {
	// need to map it to int for better comparison, otherwise go will evaluate the string unicode value.
	return utils.Map(strings.Split(set, "-"), func(t string) int {
		i, err := strconv.Atoi(t)
		if err == nil {
			return i
		}
		return 0
	})
}

func fullyContains(set []string) bool {
	// the set only has 2 elements
	if len(set) != 2 {
		return false
	}

	first := parseSet(set[0])
	second := parseSet(set[1])

	if first[0] <= second[0] && first[1] >= second[1] {
		return true
	}
	if second[0] <= first[0] && second[1] >= first[1] {
		return true
	}
	return false
}
