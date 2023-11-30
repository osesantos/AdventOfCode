package day2

import (
	"main/utils"
	"strconv"
)

// A-Rock, B-Paper, C-Scissors
// X-Rock, Y-Paper, Z-Scissors
//
// Rock - 1, Paper - 2, Scissors - 3
// Win - 6, Draw - 3, Lost - 0
//
// total score?

func Part1(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	var scores utils.IntSlice
	for _, l := range lines {
		scores = append(scores, getScore(l, func(op rune, my rune) rune {
			return my
		}))
	}
	return strconv.Itoa(scores.Sum())
}

func getScore(line string, parseScoreToPlay func(op rune, my rune) rune) int {
	runes := []rune(line)
	opPlay := runes[0]
	myPlay := parseScoreToPlay(opPlay, runes[2])
	if won(opPlay, myPlay) {
		if score, ok := playScores[myPlay]; ok {
			return score + 6
		}
	}

	if draw(opPlay, myPlay) {
		if score, ok := playScores[myPlay]; ok {
			return score + 3
		}
	}

	if score, ok := playScores[myPlay]; ok {
		return score + 0
	}

	return 0
}

var (
	playScores = map[rune]int{
		'X': 1,
		'Y': 2,
		'Z': 3,
	}
)

func won(op rune, my rune) bool {
	if (my == 'X' && op == 'C') || (my == 'Y' && op == 'A') || (my == 'Z' && op == 'B') {
		return true
	}
	return false
}

func draw(op rune, my rune) bool {
	if (my == 'X' && op == 'A') || (my == 'Y' && op == 'B') || (my == 'Z' && op == 'C') {
		return true
	}
	return false
}
