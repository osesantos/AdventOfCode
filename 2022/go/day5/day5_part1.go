package day5

import (
	"main/utils"
	"strconv"
	"strings"
)

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
// check this https://github.com/BrianMwangi21/aoc2022/blob/main/day5/main.go

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}
	//stacks := cleanMatrix(transpose(invert(convertToRune(getFirstLines(lines))), 20))
	//firstLines := convertToString(stacks)
	//return strconv.Itoa(len(firstLines))

	firstLines := getFirstLines(lines)
	stacks := convertToColumns(firstLines)
	return strconv.Itoa(len(stacks))
}

func isEmptyLine(line string) bool {
	return line == ""
}

func numberOfStacks(lines []string) int {
	for i, line := range lines {
		if lines[i+1] == "" {
			return len([]rune(strings.ReplaceAll(line, " ", "")))
		}
	}
	return 0
}

func getFirstLines(lines []string) (firstLines []string) {
	for i, l := range lines {
		if isEmptyLine(lines[i]) {
			return
		}
		firstLines = append(firstLines, l)
	}
	return
}

func invert(lines [][]rune) [][]rune {
	var inverted [][]rune
	utils.ReverseForEach(lines, func(ia int, a []rune) {
		var invertedLine []rune
		utils.ForEach(a, func(ib int, b rune) {
			invertedLine = append(invertedLine, b)
		})
		inverted = append(inverted, invertedLine)
	})

	return inverted
}

func transpose(matrix [][]rune, n int) [][]rune {
	transposed := make([][]rune, n)
	for i := range transposed {
		transposed[i] = make([]rune, n)
	}

	for i := 0; i <= n; i++ {
		for j := 0; j <= n; j++ {
			if i < len(matrix) && j < len(matrix[i]) {
				if matrix[i][j] != '[' && matrix[i][j] != ']' && matrix[i][j] != ' ' {
					transposed[j][i] = matrix[i][j]
				}
			}
		}
	}

	return transposed
}

func convertToRune(lines []string) [][]rune {
	var lineRune [][]rune
	utils.ForEach(lines, func(index int, line string) {
		lineRune = append(lineRune, []rune(line))
	})
	return lineRune
}

func convertToString(runes [][]rune) (r []string) {
	utils.ForEach(runes, func(_ int, a []rune) {
		rs := ""
		utils.ForEach(a, func(_ int, b rune) {
			rs += string(b)
		})
		r = append(r, rs)
	})
	return
}

func cleanMatrix(m [][]rune) (r [][]rune) {
	for _, v := range m {
		if v[0] != 0 {
			r = append(r, v)
		}
	}
	return
}

func convertToColumns(lines []string) [][]string {
	columns := make([][]string, len(lines[0])-1)

	for i := 0; i < len(lines[0])-1; i++ {
		columns[i] = make([]string, len(lines))
		for j, line := range lines {
			s := strings.TrimSpace(string(line[i]))
			if s != "[" && s != "]" && s != "" {
				columns[i][j] = strings.TrimSpace(string(line[i]))
			}
		}
	}

	return columns
}
