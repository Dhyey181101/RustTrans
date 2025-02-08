package test

var ()

const ()

func processLCSBacktrackAll(str1, str2 string, lcsMatrix [][]int, m, n int) StringHashMap {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	// Map containing all commons substrings (Hash set builded from map)
	substrings := make(StringHashMap)

	if m == 0 || n == 0 {
		substrings[""] = struct{}{}
	} else if runeStr1[m-1] == runeStr2[n-1] {
		for key := range processLCSBacktrackAll(str1, str2, lcsMatrix, m-1, n-1) {
			substrings[key+string(runeStr1[m-1])] = struct{}{}
		}
	} else {
		if lcsMatrix[m-1][n] >= lcsMatrix[m][n-1] {
			substrings.AddAll(processLCSBacktrackAll(str1, str2, lcsMatrix, m-1, n))
		}
		if lcsMatrix[m][n-1] >= lcsMatrix[m-1][n] {
			substrings.AddAll(processLCSBacktrackAll(str1, str2, lcsMatrix, m, n-1))
		}
	}

	return substrings
}

func (m StringHashMap) AddAll(srcMap StringHashMap) {
	for key := range srcMap {
		m[key] = struct{}{}
	}
}

// StringHashMap is HashMap substitute for string
type StringHashMap map[string]struct{}
