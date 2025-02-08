package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (addenda14 *moov_io_ach_Addenda14) EntryDetailSequenceNumberField() string {
	return addenda14.numericField(addenda14.EntryDetailSequenceNumber, 7)
}

func (c *moov_io_ach_converters) numericField(n int, max uint) string {
	s := strconv.FormatInt(int64(n), 10)
	if l := uint(len(s)); l > max {
		return s[l-max:]
	} else {
		m := int(max - l)
		pad, exists := moov_io_ach_stringZeros[m]
		if exists {
			return pad + s
		}
		// slow path
		return strings.Repeat("0", m) + s
	}
}

func moov_io_ach_populateMap(max int, zero string) map[int]string {
	out := make(map[int]string, max)
	for i := 0; i < max; i++ {
		out[i] = strings.Repeat(zero, i)
	}
	return out
}

type moov_io_ach_Addenda14 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda14 types code '14'

	// Name of the Receiver's bank

	// Receiving DFI Identification Number Qualifier
	// The 2-digit code that identifies the numbering scheme used in the
	// Receiving DFI Identification Number field:
	// 01 = National Clearing System
	// 02 = BIC Code
	// 03 = IBAN Code

	// Receiving DFI Identification
	// This field contains the bank identification number of the DFI at which the
	// Receiver maintains his account.

	// Receiving DFI Branch Country Code
	// USb” = United States
	//(“b” indicates a blank space)
	// This 3 position field contains a 2-character code as approved by the International
	// Organization for Standardization (ISO) used to identify the country in which the
	// branch of the bank that receives the entry is located. Values for other countries can
	// be found on the International Organization for Standardization website: www.iso.org

	// EntryDetailSequenceNumber contains the ascending sequence number section of the Entry
	// Detail or Corporate Entry Detail Record's trace number This number is
	// the same as the last seven digits of the trace number of the related
	// Entry Detail Record or Corporate Entry Detail Record.
	EntryDetailSequenceNumber int `json:"entryDetailSequenceNumber"`
	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
