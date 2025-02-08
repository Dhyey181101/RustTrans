package test 

import (
    "math"
)

var (
)

const (
)

func QgramDistanceCustomNgram(splittedStr1, splittedStr2 map[string]int) int {
	union := make(map[string]int)
	for i := range splittedStr1 {
		union[i] = 0
	}
	for i := range splittedStr2 {
		union[i] = 0
	}

	res := 0
	for i := range union {
		res += int(math.Abs(float64(splittedStr1[i] - splittedStr2[i])))
	}

	return res
}

