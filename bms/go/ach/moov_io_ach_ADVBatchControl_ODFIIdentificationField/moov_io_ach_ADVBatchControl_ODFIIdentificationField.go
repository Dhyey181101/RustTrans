package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (bc *moov_io_ach_ADVBatchControl) ODFIIdentificationField() string {
	return bc.stringField(bc.ODFIIdentification, 8)
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

type moov_io_ach_ADVBatchControl struct {
	// ID is a client defined string used as a reference to this record.

	// This should be the same as BatchHeader ServiceClassCode for ADV: AutomatedAccountingAdvices.

	// EntryAddendaCount is a tally of each Entry Detail Record and each Addenda
	// Record processed, within either the batch or file as appropriate.

	// validate the Receiving DFI Identification in each Entry Detail Record is hashed
	// to provide a check against inadvertent alteration of data contents due
	// to hardware failure or program error
	//
	// In this context the Entry Hash is the sum of the corresponding fields in the
	// Entry Detail Records on the file.

	// TotalDebitEntryDollarAmount Contains accumulated Entry debit totals within the batch.

	// TotalCreditEntryDollarAmount Contains accumulated Entry credit totals within the batch.

	// ACHOperatorData is an alphanumeric code used to identify an ACH Operator

	// ODFIIdentification the routing number is used to identify the DFI originating entries within a given branch.
	ODFIIdentification string `json:"ODFIIdentification"`
	// BatchNumber this number is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by record.

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
