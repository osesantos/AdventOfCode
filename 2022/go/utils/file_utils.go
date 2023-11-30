package utils

import (
	"bufio"
	"os"
	"path/filepath"
)

const inputDir = "input"

func ReadLines(filename string) (StringArray, error) {
	path := filepath.Join(inputDir, filename+".txt")
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer func(f *os.File) {
		_ = f.Close()
	}(f)

	var lines StringArray
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err = scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}
