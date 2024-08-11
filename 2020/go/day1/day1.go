package day1

import (
	"adventofcode-2020/model"
)

func Day1Part1(input model.Input) int {
	var report []int
	report = input.ParseArray()

	entry1 := 0
	entry2 := 0

	for _, a := range report {
		for _, b := range report {
			if a+b == 2020 {
				entry1 = a
				entry2 = b
			}
		}
	}

	return entry1 * entry2
}

func Day1Part2(input model.Input) int {
	var report []int
	report = input.ParseArray()

	entry1 := 0
	entry2 := 0
	entry3 := 0

	for _, a := range report {
		for _, b := range report {
			for _, c := range report {
				if a+b+c == 2020 {
					entry1 = a
					entry2 = b
					entry3 = c
				}
			}
		}
	}

	return entry1 * entry2 * entry3
}
