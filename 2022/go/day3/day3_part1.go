package day3

import (
	"errors"
	"main/utils"
	"regexp"
	"strconv"
	"strings"
)

// priorities:
// a-Z = 1-26
// A-Z = 27-52
//
// Find common element in both half, give it a priority, then sum all of them.

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	var halfLine [][][]rune
	for _, l := range lines {
		halfLine = append(halfLine, splitInHalf(l))
	}

	var dups []rune
	for _, l := range halfLine {
		dup, errDup := findDup(l[0], l[1])
		if errDup == nil {
			dups = append(dups, dup)
		}
	}

	priorities := parseToPriority(dups)

	return strconv.Itoa(priorities.Sum())
}

func splitInHalf(line string) [][]rune {
	runes := []rune(line)
	halfI := len(runes) / 2
	return [][]rune{runes[:halfI], runes[halfI:]}
}

func findDup(a []rune, b []rune) (rune, error) {
	for _, r := range a {
		if strings.ContainsRune(string(b), r) {
			return r, nil
		}
	}
	return 0, errors.New("dup not found")
}

func parseToPriority(runes []rune) utils.IntSlice {
	var priorities utils.IntSlice
	for _, r := range runes {
		match, err := regexp.MatchString(`[A-Z]`, string(r))
		if err == nil && match {
			priorities = append(priorities, parseCapitalRune(r))
		}

		match, err = regexp.MatchString(`[a-z]`, string(r))
		if err == nil && match {
			priorities = append(priorities, parseRegularRune(r))
		}
	}
	return priorities
}

func parseCapitalRune(r rune) int {
	return int(r) - 38
}

func parseRegularRune(r rune) int {
	return int(r) - 96
}
