package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (addenda02 *moov_io_ach_Addenda02) TraceNumberField() string {
	return addenda02.stringField(addenda02.TraceNumber, 15)
}

func (c *moov_io_ach_converters) stringField(s string, max uint) string {
	ln := uint(utf8.RuneCountInString(s))
	if ln > max {
		return s[:max]
	}

	// Pad with preallocated string
	m := int(max - ln)
	pad, exists := moov_io_ach_stringZeros[m]
	if exists {
		return pad + s
	}
	// slow path
	return strings.Repeat("0", m) + s
}

func moov_io_ach_populateMap(max int, zero string) map[int]string {
	out := make(map[int]string, max)
	for i := 0; i < max; i++ {
		out[i] = strings.Repeat(zero, i)
	}
	return out
}

type moov_io_ach_Addenda02 struct {
	// ID is a client defined string used as a reference to this record.

	// TypeCode Addenda02 type code '02'

	// ReferenceInformationOne may be used for additional reference numbers, identification numbers,
	// or codes that the merchant needs to identify the particular transaction or customer.

	// ReferenceInformationTwo  may be used for additional reference numbers, identification numbers,
	// or codes that the merchant needs to identify the particular transaction or customer.

	// TerminalIdentificationCode identifies an Electronic terminal with a unique code that allows
	// a terminal owner and/or switching network to identify the terminal at which an Entry originated.

	// TransactionSerialNumber is assigned by the terminal at the time the transaction is originated.  The
	// number, with the Terminal Identification Code, serves as an audit trail for the transaction and is
	// usually assigned in ascending sequence.

	// TransactionDate expressed MMDD identifies the date on which the transaction occurred.

	// AuthorizationCodeOrExpireDate indicates the code that a card authorization center has
	// furnished to the merchant.

	// Terminal Location identifies the specific location of a terminal (i.e., street names of an
	// intersection, address, etc.) in accordance with the requirements of Regulation E.

	// TerminalCity Identifies the city in which the electronic terminal is located.

	// TerminalState Identifies the state in which the electronic terminal is located

	// TraceNumber Standard Entry Detail Trace Number
	//
	// Use TraceNumberField for a properly formatted string representation.
	TraceNumber string `json:"traceNumber,omitempty"`
	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
