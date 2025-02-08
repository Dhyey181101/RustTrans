package test 

import (
)

var (
)

const (
)

func OSADamerauLevenshteinDistance(str1, str2 string) int {
	// Convert string parameters to rune arrays to be compatible with non-ASCII
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Get and store length of these strings
	runeStr1len := len(runeStr1)
	runeStr2len := len(runeStr2)
	if runeStr1len == 0 {
		return runeStr2len
	} else if runeStr2len == 0 {
		return runeStr1len
	} else if Equal(runeStr1, runeStr2) {
		return 0
	} else if runeStr1len < runeStr2len {
		return OSADamerauLevenshteinDistance(str2, str1)
	}

	// 2D Array
	row := Min(runeStr1len+1, 3)
	matrix := make([][]int, row)
	for i := 0; i < row; i++ {
		matrix[i] = make([]int, runeStr2len+1)
		matrix[i][0] = i
	}

	for j := 0; j <= runeStr2len; j++ {
		matrix[0][j] = j
	}

	var count int
	for i := 1; i <= runeStr1len; i++ {
		matrix[i%3][0] = i
		for j := 1; j <= runeStr2len; j++ {
			if runeStr1[i-1] == runeStr2[j-1] {
				count = 0
			} else {
				count = 1
			}

			matrix[i%3][j] = Min(Min(matrix[(i-1)%3][j]+1, matrix[i%3][j-1]+1),
				matrix[(i-1)%3][j-1]+count) // insertion, deletion, substitution
			if i > 1 && j > 1 && runeStr1[i-1] == runeStr2[j-2] && runeStr1[i-2] == runeStr2[j-1] {
				matrix[i%3][j] = Min(matrix[i%3][j], matrix[(i-2)%3][j-2]+1) // translation
			}
		}
	}
	return matrix[runeStr1len%3][runeStr2len]
}

func Equal(a, b []rune) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

func Min(a int, b int) int {
	if b < a {
		return b
	}
	return a
}

