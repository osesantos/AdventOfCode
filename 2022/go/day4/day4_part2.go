package day4

import (
	"main/utils"
	"strconv"
)

func Part2(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	parsedLines := utils.Map(lines, func(t string) []string {
		return parseLine(t)
	})

	filteredLines := utils.Filter(parsedLines, func(i []string) bool {
		return partiallyContains(i)
	})

	return strconv.Itoa(len(filteredLines))
}

func fillSlice(a int, b int) []int {
	var s []int
	for i := a; i <= b; i++ {
		s = append(s, i)
	}
	return s
}

func partiallyContains(set []string) bool {
	// the set only has 2 elements
	if len(set) != 2 {
		return false
	}

	first := parseSet(set[0])
	second := parseSet(set[1])

	firstS := fillSlice(first[0], first[1])
	secondS := fillSlice(second[0], second[1])

	r := false
	utils.ForEach(firstS, func(_, a int) {
		utils.ForEach(secondS, func(_, b int) {
			if a == b {
				r = true
			}
		})
	})

	return r
}
