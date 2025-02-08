package test

var ()

const ()

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
