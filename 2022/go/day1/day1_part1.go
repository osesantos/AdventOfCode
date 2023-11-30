package day1

import (
	"main/utils"
	"strconv"
	"strings"
)

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	topCal := 0
	calCount := 0
	lines.ForEach(func(index int, value string) {
		if strings.Trim(value, " ") == "" {
			if topCal < calCount {
				topCal = calCount
			}
			calCount = 0
			return
		}
		if i, errConv := strconv.Atoi(value); errConv == nil {
			calCount = calCount + i
		}
	})

	return strconv.Itoa(topCal)
}
