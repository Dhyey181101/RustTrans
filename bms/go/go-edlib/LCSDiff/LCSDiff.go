package test 

import (
    "errors"
)

var (
)

const (
)

func LCSDiff(str1, str2 string) ([]string, error) {
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	if len(runeStr1) == 0 || len(runeStr2) == 0 {
		return nil, errors.New("Can't process LCS diff with empty string")
	} else if Equal(runeStr1, runeStr2) {
		return []string{str1}, nil
	}

	diff := processLCSDiff(str1, str2, lcsProcess(runeStr1, runeStr2), len(runeStr1), len(runeStr2))
	return diff, nil
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

func lcsProcess(runeStr1, runeStr2 []rune) [][]int {
	// 2D Array that will contain str1 and str2 LCS
	lcsMatrix := make([][]int, len(runeStr1)+1)
	for i := 0; i <= len(runeStr1); i++ {
		lcsMatrix[i] = make([]int, len(runeStr2)+1)
		for j := 0; j <= len(runeStr2); j++ {
			lcsMatrix[i][j] = 0
		}
	}

	for i := 1; i <= len(runeStr1); i++ {
		for j := 1; j <= len(runeStr2); j++ {
			if runeStr1[i-1] == runeStr2[j-1] {
				lcsMatrix[i][j] = lcsMatrix[i-1][j-1] + 1
			} else {
				lcsMatrix[i][j] = Max(lcsMatrix[i][j-1], lcsMatrix[i-1][j])
			}
		}
	}

	return lcsMatrix
}

func Max(a int, b int) int {
	if b > a {
		return b
	}
	return a
}

func processLCSDiff(str1 string, str2 string, lcsMatrix [][]int, m, n int) []string {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	diff := make([]string, 2)

	if m > 0 && n > 0 && runeStr1[m-1] == runeStr2[n-1] {
		diff = processLCSDiff(str1, str2, lcsMatrix, m-1, n-1)
		diff[0] = diff[0] + " " + string(runeStr1[m-1])
		diff[1] = diff[1] + "  "
		return diff
	} else if n > 0 && (m == 0 || lcsMatrix[m][n-1] > lcsMatrix[m-1][n]) {
		diff = processLCSDiff(str1, str2, lcsMatrix, m, n-1)
		diff[0] = diff[0] + " " + string(runeStr2[n-1])
		diff[1] = diff[1] + " +"
		return diff
	} else if m > 0 && (n == 0 || lcsMatrix[m][n-1] <= lcsMatrix[m-1][n]) {
		diff = processLCSDiff(str1, str2, lcsMatrix, m-1, n)
		diff[0] = diff[0] + " " + string(runeStr1[m-1])
		diff[1] = diff[1] + " -"
		return diff
	}

	return diff
}

