package test

import (
	"errors"
	"fmt"
	"math"
	"unicode/utf8"
)

var ()

const (
	moov_io_ach_GLPrenoteDebit = 48
)

func moov_io_ach_CheckRoutingNumber(routingNumber string) error {
	if routingNumber == "" {
		return errors.New("no routing number provided")
	}
	if n := utf8.RuneCountInString(routingNumber); n != 9 {
		return fmt.Errorf("invalid routing number length of %d", n)
	}

	check := fmt.Sprintf("%d", moov_io_ach_CalculateCheckDigit(routingNumber))
	last := string(routingNumber[len(routingNumber)-1])
	if check != last {
		return fmt.Errorf("routing number checksum mismatch: expected %s but got %s", check, last)
	}
	return nil
}

func moov_io_ach_CalculateCheckDigit(routingNumber string) int {
	if n := utf8.RuneCountInString(routingNumber); n != 8 && n != 9 {
		return -1
	}

	var sum int
	for i, r := range routingNumber {
		// Don't process check digit of routing number
		if i >= 8 {
			break
		}

		// Reject anything that's not a digit
		if r < '0' || r > '9' {
			return -1 // only digits are allowed
		}

		// Calculate the check digit
		var n int32 = (r - '0')

		switch i {
		case 0, 3, 6:
			sum += int(n * 3)
		case 1, 4, 7:
			sum += int(n * 7)
		case 2, 5:
			sum += int(n)
		}
	}

	return moov_io_ach_roundUp10(sum) - sum
}

func moov_io_ach_roundUp10(n int) int {
	return int(math.Ceil(float64(n)/10.0)) * 10
}
