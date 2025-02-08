package test 

import (
    "math"
)

var (
)

const (
)

func QgramSimilarity(str1, str2 string, splitLength int) float32 {
	splittedStr1 := Shingle(str1, splitLength)
	splittedStr2 := Shingle(str2, splitLength)
	res := float32(QgramDistanceCustomNgram(splittedStr1, splittedStr2))
	return 1 - (res / float32(len(splittedStr1)+len(splittedStr2)))
}

func Shingle(s string, k int) map[string]int {
	m := make(map[string]int)
	if s != "" && k != 0 {
		runeS := []rune(s)

		for i := 0; i < len(runeS)-k+1; i++ {
			m[string(runeS[i:i+k])]++
		}
	}
	return m
}

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

