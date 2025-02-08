package test 

import (
)

var (
)

const (
)

func DamerauLevenshteinDistance(str1, str2 string) int {
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
	}

	// Create alphabet based on input strings
	da := make(map[rune]int)
	for i := 0; i < runeStr1len; i++ {
		da[runeStr1[i]] = 0
	}
	for i := 0; i < runeStr2len; i++ {
		da[runeStr2[i]] = 0
	}

	// 2D Array for distance matrix : matrix[0..str1.length+2][0..s2.length+2]
	matrix := make([][]int, runeStr1len+2)
	for i := 0; i <= runeStr1len+1; i++ {
		matrix[i] = make([]int, runeStr2len+2)
		for j := 0; j <= runeStr2len+1; j++ {
			matrix[i][j] = 0
		}
	}

	// Maximum possible distance
	maxDist := runeStr1len + runeStr2len

	// Initialize matrix
	matrix[0][0] = maxDist
	for i := 0; i <= runeStr1len; i++ {
		matrix[i+1][0] = maxDist
		matrix[i+1][1] = i
	}
	for i := 0; i <= runeStr2len; i++ {
		matrix[0][i+1] = maxDist
		matrix[1][i+1] = i
	}

	// Process edit distance
	var cost int
	for i := 1; i <= runeStr1len; i++ {
		db := 0
		for j := 1; j <= runeStr2len; j++ {
			i1 := da[runeStr2[j-1]]
			j1 := db
			if runeStr1[i-1] == runeStr2[j-1] {
				cost = 0
				db = j
			} else {
				cost = 1
			}

			matrix[i+1][j+1] = Min(
				Min(
					matrix[i+1][j]+1,  // Addition
					matrix[i][j+1]+1), // Deletion
				Min(
					matrix[i][j]+cost, // Substitution
					matrix[i1][j1]+(i-i1-1)+1+(j-j1-1))) // Transposition
		}

		da[runeStr1[i-1]] = i
	}

	return matrix[runeStr1len+1][runeStr2len+1]
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

