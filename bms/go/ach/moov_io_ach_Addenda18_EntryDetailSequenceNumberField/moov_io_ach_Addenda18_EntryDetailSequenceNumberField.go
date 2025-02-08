package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (addenda18 *moov_io_ach_Addenda18) EntryDetailSequenceNumberField() string {
	return addenda18.numericField(addenda18.EntryDetailSequenceNumber, 7)
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

type moov_io_ach_Addenda18 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda18 types code '18'

	// ForeignCorrespondentBankName contains the name of the Foreign Correspondent Bank

	// Foreign Correspondent Bank Identification Number Qualifier contains a 2-digit code that
	// identifies the numbering scheme used in the Foreign Correspondent Bank Identification Number
	// field. Code values for this field are:
	// “01” = National Clearing System
	// “02” = BIC Code
	// “03” = IBAN Code

	// Foreign Correspondent Bank Identification Number contains the bank ID number of the Foreign
	// Correspondent Bank

	// Foreign Correspondent Bank Branch Country Code contains the two-character code, as approved by
	// the International Organization for Standardization (ISO), to identify the country in which the
	// branch of the Foreign Correspondent Bank is located. Values can be found on the International
	// Organization for Standardization website: www.iso.org

	// SequenceNumber is consecutively assigned to each Addenda18 Record following
	// an Entry Detail Record. The first addenda18 sequence number must always
	// be a "1".

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
