package main

import (
	"adventofcode-2020/day1"
	"adventofcode-2020/day2"
	"adventofcode-2020/day3"
	"adventofcode-2020/day4"
	"bufio"
	"fmt"
	"os"
)

func getInputLines(path string) []string {
	readFile, err := os.Open(path)
	if err != nil {
		panic(fmt.Sprintf("Unable to open file %s", path))
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	readFile.Close()

	return fileLines
}

func main() {
	fmt.Println("#########################################################################")
	fmt.Println("                        adventofcode-2020")
	fmt.Println("#########################################################################")

	// Day 1
	fmt.Println("Day1.1 - ", day1.Day1Part1(getInputLines("day1/input")))
	fmt.Println("Day1.2 - ", day1.Day1Part2(getInputLines("day1/input")))

	// Day 2
	fmt.Println("Day2.1 - ", day2.Day2Part1(getInputLines("day2/input")))
	fmt.Println("Day2.2 - ", day2.Day2Part2(getInputLines("day2/input")))

	// Day 3
	fmt.Println("Day3.1 - ", day3.Day3Part1(getInputLines("day3/input")))
	fmt.Println("Day3.2 - ", day3.Day3Part2(getInputLines("day3/input")))

	// Day 4
	fmt.Println("Day4.1 - ", day4.Day4Part1(getInputLines("day4/input")))
	fmt.Println("Day4.2 - ", day4.Day4Part2(getInputLines("day4/input")))
}
