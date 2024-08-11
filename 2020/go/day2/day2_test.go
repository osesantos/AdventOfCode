package day2

import (
	"testing"
)

func TestDay2Part1(t *testing.T) {
	// Test cases
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{
			"1-3 a: abcde",
			"1-3 b: cdefg",
			"2-9 c: ccccccccc",
		}, 2},
	}

	// Test
	for _, c := range cases {
		got := Day2Part1(c.in)
		if got != c.want {
			t.Errorf("Day1Part1(%v) == %v, expected %v", c.in, got, c.want)
		}
	}
}

func TestDay2Part2(t *testing.T) {
	// Test cases
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{
			"1-3 a: abcde",
			"1-3 b: cdefg",
			"2-9 c: ccccccccc",
		}, 1},
	}

	// Test
	for _, c := range cases {
		got := Day2Part2(c.in)
		if got != c.want {
			t.Errorf("Day1Part1(%v) == %v, expected %v", c.in, got, c.want)
		}
	}
}
