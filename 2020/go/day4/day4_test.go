package day4

import (
	"testing"
)

func TestDay3Part1(t *testing.T) {
	// Test cases
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{
			"..##.......",
			"#...#...#..",
			".#....#..#.",
			"..#.#...#.#",
			".#...##..#.",
			"..#.##.....",
			".#.#.#....#",
			".#........#",
			"#.##...#...",
			"#...##....#",
			".#..#...#.#",
		}, 7},
	}

	// Test
	for _, c := range cases {
		got := Day4Part1(c.in)
		if got != c.want {
			t.Errorf("Day1Part1(%v) == %v, expected %v", c.in, got, c.want)
		}
	}
}
