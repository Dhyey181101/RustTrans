package test 

import (
)

var (
)

const (
)

func SorensenDiceCoefficient(str1, str2 string, splitLength int) float32 {
	if str1 == "" && str2 == "" {
		return 0
	}
	shingle1 := Shingle(str1, splitLength)
	shingle2 := Shingle(str2, splitLength)

	intersection := float32(0)
	for i := range shingle1 {
		if _, ok := shingle2[i]; ok {
			intersection++
		}
	}
	return 2.0 * intersection / float32(len(shingle1)+len(shingle2))
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

