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

func (addenda05 *moov_io_ach_Addenda05) String() string {
	if addenda05 == nil {
		return ""
	}

	var buf strings.Builder
	buf.Grow(94)
	buf.WriteString(moov_io_ach_entryAddendaPos)
	buf.WriteString(addenda05.TypeCode)
	buf.WriteString(addenda05.PaymentRelatedInformationField())
	buf.WriteString(addenda05.SequenceNumberField())
	buf.WriteString(addenda05.EntryDetailSequenceNumberField())
	return buf.String()
}

func (addenda05 *moov_io_ach_Addenda05) PaymentRelatedInformationField() string {
	return addenda05.alphaField(addenda05.PaymentRelatedInformation, 80)
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

func (addenda05 *moov_io_ach_Addenda05) SequenceNumberField() string {
	return addenda05.numericField(addenda05.SequenceNumber, 4)
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

func (addenda05 *moov_io_ach_Addenda05) EntryDetailSequenceNumberField() string {
	return addenda05.numericField(addenda05.EntryDetailSequenceNumber, 7)
}

func moov_io_ach_populateMap(max int, zero string) map[int]string {
	out := make(map[int]string, max)
	for i := 0; i < max; i++ {
		out[i] = strings.Repeat(zero, i)
	}
	return out
}

type moov_io_ach_Addenda05 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda05 types code '05'
	TypeCode string `json:"typeCode"`
	// PaymentRelatedInformation
	PaymentRelatedInformation string `json:"paymentRelatedInformation"`
	// SequenceNumber is consecutively assigned to each Addenda05 Record following
	// an Entry Detail Record. The first addenda05 sequence number must always
	// be a "1".
	SequenceNumber int `json:"sequenceNumber"`
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
