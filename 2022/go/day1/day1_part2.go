package day1

import (
	"main/utils"
	"sort"
	"strconv"
	"strings"
)

func Part2(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	var topCals []int
	calCount := 0
	lines.ForEach(func(index int, value string) {
		if i, errConv := strconv.Atoi(value); errConv == nil {
			calCount = calCount + i
		}

		if strings.Trim(value, " ") == "" || index+1 == len(lines) {
			topCals = append(topCals, calCount)
			calCount = 0
			return
		}
	})

	sort.Slice(topCals, func(i, j int) bool {
		return topCals[i] > topCals[j]
	})

	return strconv.Itoa(Sum(topCals[:3]))
}

func Sum(slice []int) int {
	sum := 0
	for _, v := range slice {
		sum += v
	}
	return sum
}
