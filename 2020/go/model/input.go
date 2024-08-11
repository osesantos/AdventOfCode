package model

import "strconv"

type Input []string

func (input Input) ParseArray() []int {
	var newArray []int
	for _, e := range input {
		i, err := strconv.Atoi(e)
		if err != nil {
			panic("unable to parse []string to []int")
		}

		newArray = append(newArray, i)
	}

	return newArray
}
