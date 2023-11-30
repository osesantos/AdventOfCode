package utils

type StringExtensions interface {
	ForEach(func(...any))
}

type StringArray []string

func (s StringArray) ForEach(f func(index int, value string)) {
	for i, v := range s {
		f(i, v)
	}
}

func ForEach[T any](data []T, f func(index int, value T)) {
	for i, v := range data {
		f(i, v)
	}
}

func ReverseForEach[T any](data []T, f func(index int, value T)) {
	for i, _ := range data {
		f(len(data)-1-i, data[len(data)-1-i])
	}
}

func Map[T, U any](data []T, f func(T) U) []U {
	res := make([]U, 0, len(data))

	for _, e := range data {
		res = append(res, f(e))
	}

	return res
}

func Filter[T any](data []T, f func(T) bool) []T {
	filtered := make([]T, 0, len(data))
	for _, e := range data {
		if f(e) {
			filtered = append(filtered, e)
		}
	}
	return filtered
}

type IntSlice []int

func (is IntSlice) Sum() int {
	var s int
	for _, v := range is {
		s += v
	}
	return s
}
