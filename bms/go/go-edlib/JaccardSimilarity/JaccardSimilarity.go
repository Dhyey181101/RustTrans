package test

import (
	"strings"
)

var ()

const ()

func JaccardSimilarity(str1, str2 string, splitLength int) float32 {
	if str1 == "" || str2 == "" {
		return 0
	}

	// Split string before rune conversion for jaccard calculation
	// If splitLength == 0 then split on whitespaces
	// Else use shingle algorithm
	var splittedStr1, splittedStr2 []string
	if splitLength == 0 {
		splittedStr1 = strings.Split(str1, " ")
		splittedStr2 = strings.Split(str2, " ")
	} else {
		splittedStr1 = ShingleSlice(str1, splitLength)
		splittedStr2 = ShingleSlice(str2, splitLength)
	}

	// Conversion of splitted string into rune array
	runeStr1 := make([][]rune, len(splittedStr1))
	for i, str := range splittedStr1 {
		runeStr1[i] = []rune(str)
	}
	runeStr2 := make([][]rune, len(splittedStr2))
	for i, str := range splittedStr2 {
		runeStr2[i] = []rune(str)
	}

	// Create union keywords slice between input strings
	unionStr := union(splittedStr1, splittedStr2)
	jacc := float32(len(runeStr1) + len(runeStr2) - len(unionStr))

	return jacc / float32(len(unionStr))
}

func ShingleSlice(s string, k int) []string {
	var out []string
	m := make(map[string]int)
	if s != "" && k != 0 {
		runeS := []rune(s)
		for i := 0; i < len(runeS)-k+1; i++ {
			m[string(runeS[i:i+k])]++
		}
		for k := range m {
			out = append(out, k)
		}
	}
	return out
}

func union(a, b []string) [][]rune {
	m := make(map[string]bool)
	for _, item := range a {
		m[item] = true
	}
	for _, item := range b {
		if _, ok := m[item]; !ok {
			a = append(a, item)
		}
	}

	// Convert a to rune matrix (with x -> words and y -> characters)
	out := make([][]rune, len(a))
	for i, word := range a {
		out[i] = []rune(word)
	}
	return out
}
