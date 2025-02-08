package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (addenda13 *moov_io_ach_Addenda13) EntryDetailSequenceNumberField() string {
	return addenda13.numericField(addenda13.EntryDetailSequenceNumber, 7)
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

type moov_io_ach_Addenda13 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda13 types code '13'

	// Originating DFI Name
	// For Outbound IAT Entries, this field must contain the name of the U.S. ODFI.
	// For Inbound IATs: Name of the foreign bank providing funding for the payment transaction

	// Originating DFI Identification Number Qualifier
	// For Inbound IATs: The 2-digit code that identifies the numbering scheme used in the
	// Foreign DFI Identification Number field:
	// 01 = National Clearing System
	// 02 = BIC Code
	// 03 = IBAN Code

	// Originating DFI Identification
	// This field contains the routing number that identifies the U.S. ODFI initiating the entry.
	// For Inbound IATs: This field contains the bank ID number of the Foreign Bank providing funding
	// for the payment transaction.

	// Originating DFI Branch Country Code
	// USb” = United States
	//(“b” indicates a blank space)
	// For Inbound IATs: This 3 position field contains a 2-character code as approved by the
	// International Organization for Standardization (ISO) used to identify the country in which
	// the branch of the bank that originated the entry is located. Values for other countries can
	// be found on the International Organization for Standardization website: www.iso.org.

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
