package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (bc *moov_io_ach_BatchControl) BatchNumberField() string {
	return bc.numericField(bc.BatchNumber, 7)
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

type moov_io_ach_BatchControl struct {
	// ID is a client defined string used as a reference to this record.

	// ServiceClassCode ACH Mixed Debits and Credits '200'
	// ACH Credits Only '220'
	// ACH Debits Only '225'
	// Constants: MixedCreditsAnDebits (220), CReditsOnly 9220), DebitsOnly (225)
	// Same as 'ServiceClassCode' in BatchHeaderRecord

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

	// CompanyIdentification is an alphanumeric code used to identify an Originator
	// The Company Identification Field must be included on all
	// prenotification records and on each entry initiated pursuant to such
	// prenotification. The Company ID may begin with the ANSI one-digit
	// Identification Code Designator (ICD), followed by the identification
	// number The ANSI Identification Numbers and related Identification Code
	// Designator (ICD) are:
	//
	// IRS Employer Identification Number (EIN) "1"
	// Data Universal Numbering Systems (DUNS) "3"
	// User Assigned Number "9"

	// MessageAuthenticationCode the MAC is an eight character code derived from a special key used in
	// conjunction with the DES algorithm. The purpose of the MAC is to
	// validate the authenticity of ACH entries. The DES algorithm and key
	// message standards must be in accordance with standards adopted by the
	// American National Standards Institute. The remaining eleven characters
	// of this field are blank.

	// ODFIIdentification the routing number is used to identify the DFI originating entries within a given branch.

	// BatchNumber this number is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by record.
	BatchNumber int `json:"batchNumber"`
	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
