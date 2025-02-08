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

func (addenda11 *moov_io_ach_Addenda11) String() string {
	if addenda11 == nil {
		return ""
	}

	var buf strings.Builder
	buf.Grow(94)
	buf.WriteString(moov_io_ach_entryAddendaPos)
	buf.WriteString(addenda11.TypeCode)
	buf.WriteString(addenda11.OriginatorNameField())
	buf.WriteString(addenda11.OriginatorStreetAddressField())
	buf.WriteString("              ")
	buf.WriteString(addenda11.EntryDetailSequenceNumberField())
	return buf.String()
}

func (addenda11 *moov_io_ach_Addenda11) OriginatorNameField() string {
	return addenda11.alphaField(addenda11.OriginatorName, 35)
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

func (addenda11 *moov_io_ach_Addenda11) OriginatorStreetAddressField() string {
	return addenda11.alphaField(addenda11.OriginatorStreetAddress, 35)
}

func (addenda11 *moov_io_ach_Addenda11) EntryDetailSequenceNumberField() string {
	return addenda11.numericField(addenda11.EntryDetailSequenceNumber, 7)
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

type moov_io_ach_Addenda11 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda11 types code '11'
	TypeCode string `json:"typeCode"`
	// Originator Name contains the originators name (your company name / name)
	OriginatorName string `json:"originatorName"`
	// Originator Street Address Contains the originators street address (your company's address / your address)
	OriginatorStreetAddress string `json:"originatorStreetAddress"`
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
