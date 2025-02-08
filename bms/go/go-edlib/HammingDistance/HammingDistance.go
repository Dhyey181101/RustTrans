package test 

import (
    "errors"
)

var (
)

const (
)

func HammingDistance(str1, str2 string) (int, error) {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	if len(runeStr1) != len(runeStr2) {
		return 0, errors.New("Undefined for strings of unequal length")
	} else if Equal(runeStr1, runeStr2) {
		return 0, nil
	}

	var counter int
	for i := 0; i < len(runeStr1); i++ {
		if runeStr1[i] != runeStr2[i] {
			counter++
		}
	}

	return counter, nil
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

