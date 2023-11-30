package day2

import (
	"errors"
	"main/utils"
	"strconv"
)

// A-Rock, B-Paper, C-Scissors
// X-Rock, Y-Paper, Z-Scissors
//
// X-Lose, Y-Draw, Z-Won
//
// Rock - 1, Paper - 2, Scissors - 3
// Win - 6, Draw - 3, Lost - 0
//
// total score?

func Part2(filename string) string {
	lines, err := utils.ReadLines(filename)
	if err != nil {
		return ""
	}

	var scores utils.IntSlice
	for _, l := range lines {
		scores = append(scores, getScore(l, func(op rune, my rune) rune {
			myPlay, errParse := convertScoreToPlay(op, my)
			if errParse != nil {
				return my
			}
			return myPlay
		}))
	}
	return strconv.Itoa(scores.Sum())
}

func convertScoreToPlay(opponent rune, my rune) (rune, error) {
	// I need to lose
	if my == 'X' {
		if opponent == 'A' {
			return 'Z', nil
		}
		if opponent == 'B' {
			return 'X', nil
		}
		if opponent == 'C' {
			return 'Y', nil
		}
	}

	// I need to win
	if my == 'Z' {
		if opponent == 'A' {
			return 'Y', nil
		}
		if opponent == 'B' {
			return 'Z', nil
		}
		if opponent == 'C' {
			return 'X', nil
		}
	}

	// I need a draw
	if opponent == 'A' {
		return 'X', nil
	}
	if opponent == 'B' {
		return 'Y', nil
	}
	if opponent == 'C' {
		return 'Z', nil
	}

	return -1, errors.New("can't parse")
}
