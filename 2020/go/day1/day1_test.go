package day1

import (
	"testing"
)

func TestDay1Part1(t *testing.T) {
	// Test cases
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{"1721", "979", "366", "299", "675", "1456"}, 514579},
	}

	// Test
	for _, c := range cases {
		got := Day1Part1(c.in)
		if got != c.want {
			t.Errorf("Day1Part1(%v) == %v, expected %v", c.in, got, c.want)
		}
	}
}

func TestDay1Part2(t *testing.T) {
	// Test cases
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{"1721", "979", "366", "299", "675", "1456"}, 241861950},
	}

	// Test
	for _, c := range cases {
		got := Day1Part2(c.in)
		if got != c.want {
			t.Errorf("Day1Part1(%v) == %v, expected %v", c.in, got, c.want)
		}
	}
}
