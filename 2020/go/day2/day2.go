package day2

import (
	"adventofcode-2020/model"
	"strconv"
	"strings"
)

type Line struct {
	policy   string
	password string
}

func parseInput(input model.Input) []Line {
	var lines []Line

	for _, e := range input {
		split := strings.Split(e, ":")
		lines = append(lines, Line{
			policy:   split[0],
			password: strings.Trim(split[1], " "),
		})
	}

	return lines
}

func Day2Part1(input model.Input) int {
	lines := parseInput(input)

	nrMatch := 0

	for _, line := range lines {
		letter := strings.Split(line.policy, " ")[1]
		least, err := strconv.Atoi(strings.Split(strings.Split(line.policy, " ")[0], "-")[0])
		most, err := strconv.Atoi(strings.Split(strings.Split(line.policy, " ")[0], "-")[1])

		if err != nil {
			panic("error converting least or most to int")
		}

		counter := 0

		for _, a := range line.password {
			if string(a) == letter {
				counter++
			}
		}

		if least <= counter && counter <= most {
			nrMatch++
		}
	}

	return nrMatch
}

func Day2Part2(input model.Input) int {
	lines := parseInput(input)

	nrMatch := 0

	for _, line := range lines {
		letter := strings.Split(line.policy, " ")[1]
		first, err := strconv.Atoi(strings.Split(strings.Split(line.policy, " ")[0], "-")[0])
		second, err := strconv.Atoi(strings.Split(strings.Split(line.policy, " ")[0], "-")[1])

		if err != nil {
			panic("error converting first or second to int")
		}

		if string(line.password[first-1]) == letter && string(line.password[second-1]) == letter {
			continue
		}

		if string(line.password[first-1]) == letter || string(line.password[second-1]) == letter {
			nrMatch++
		}
	}

	return nrMatch
}
