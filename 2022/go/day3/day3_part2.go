package day3

import (
	"errors"
	"main/utils"
	"strconv"
	"strings"
)

func Part2(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	groups := groupLines(lines)
	items := getListOfItems(groups)

	priorities := parseToPriority(items)

	return strconv.Itoa(priorities.Sum())
}

func groupLines(lines []string) [][]string {
	var groups [][]string
	var group []string
	for i, v := range lines {
		group = append(group, v)
		if ((i+1)%3 == 0 && i != 0) || (i+1) == len(lines) {
			groups = append(groups, group)
			group = []string{}
		}
	}
	return groups
}

func getListOfItems(groups [][]string) []rune {
	var items []rune
	for _, v := range groups {
		r, err := findCommonItem(v)
		if err == nil {
			items = append(items, r)
		}
	}
	return items
}

func findCommonItem(group []string) (rune, error) {
	first := []rune(group[0])
	for _, r := range first {
		if strings.ContainsRune(group[1], r) && strings.ContainsRune(group[2], r) {
			return r, nil
		}
	}
	return 0, errors.New("rune not found")

}
