package test

var ()

const ()

func processLCSBacktrack(str1, str2 string, lcsMatrix [][]int, m, n int) string {
	// Convert strings to rune array to handle no-ASCII characters
	runeStr1 := []rune(str1)
	runeStr2 := []rune(str2)

	if m == 0 || n == 0 {
		return ""
	} else if runeStr1[m-1] == runeStr2[n-1] {
		return processLCSBacktrack(str1, str2, lcsMatrix, m-1, n-1) + string(runeStr1[m-1])
	} else if lcsMatrix[m][n-1] > lcsMatrix[m-1][n] {
		return processLCSBacktrack(str1, str2, lcsMatrix, m, n-1)
	}

	return processLCSBacktrack(str1, str2, lcsMatrix, m-1, n)
}
