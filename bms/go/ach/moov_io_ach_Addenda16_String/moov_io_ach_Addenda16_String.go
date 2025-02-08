package test

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_spaceZeros  map[int]string = moov_io_ach_populateMap(94, " ")
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const (
	moov_io_ach_RecordLength    = 94
	moov_io_ach_entryAddendaPos = "7"
)

func (addenda16 *moov_io_ach_Addenda16) String() string {
	if addenda16 == nil {
		return ""
	}

	var buf strings.Builder
	buf.Grow(94)
	buf.WriteString(moov_io_ach_entryAddendaPos)
	buf.WriteString(addenda16.TypeCode)
	buf.WriteString(addenda16.ReceiverCityStateProvinceField())
	buf.WriteString(addenda16.ReceiverCountryPostalCodeField())
	buf.WriteString("              ")
	buf.WriteString(addenda16.EntryDetailSequenceNumberField())
	return buf.String()
}

func (addenda16 *moov_io_ach_Addenda16) ReceiverCityStateProvinceField() string {
	return addenda16.alphaField(addenda16.ReceiverCityStateProvince, 35)
}

func (c *moov_io_ach_converters) alphaField(s string, max uint) string {
	ln := uint(utf8.RuneCountInString(s))
	if ln > max {
		return s[:max]
	}

	m := int(max - ln)
	pad, exists := moov_io_ach_spaceZeros[m]
	if exists {
		return s + pad
	}
	// slow path
	return s + strings.Repeat(" ", int(max-ln))
}

func (addenda16 *moov_io_ach_Addenda16) ReceiverCountryPostalCodeField() string {
	return addenda16.alphaField(addenda16.ReceiverCountryPostalCode, 35)
}

func (addenda16 *moov_io_ach_Addenda16) EntryDetailSequenceNumberField() string {
	return addenda16.numericField(addenda16.EntryDetailSequenceNumber, 7)
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

type moov_io_ach_Addenda16 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda16 types code '16'
	TypeCode string `json:"typeCode"`
	// Receiver City & State / Province
	// Data elements City and State / Province  should be separated with an asterisk (*) as a delimiter
	// and the field should end with a backslash (\).
	// For example: San Francisco*CA\
	ReceiverCityStateProvince string `json:"receiverCityStateProvince"`
	// Receiver Country & Postal Code
	// Data elements must be separated by an asterisk (*) and must end with a backslash (\)
	// For example: US*10036\
	ReceiverCountryPostalCode string `json:"receiverCountryPostalCode"`
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
