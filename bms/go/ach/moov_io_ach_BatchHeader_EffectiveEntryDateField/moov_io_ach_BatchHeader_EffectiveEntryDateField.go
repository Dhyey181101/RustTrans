package test

import (
	"strings"
	"unicode/utf8"
)

var (
	moov_io_ach_spaceZeros  map[int]string = moov_io_ach_populateMap(94, " ")
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (bh *moov_io_ach_BatchHeader) EffectiveEntryDateField() string {
	// ENR records require EffectiveEntryDate to be space filled. NACHA Page OR108
	if bh.CompanyEntryDescription == "AUTOENROLL" {
		return bh.alphaField("", 6)
	}
	return bh.stringField(bh.EffectiveEntryDate, 6) // YYMMDD
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

type moov_io_ach_BatchHeader struct {
	// ID is a client defined string used as a reference to this record.

	// ServiceClassCode ACH Mixed Debits and Credits '200'
	// ACH Credits Only '220'
	// ACH Debits Only '225'

	// CompanyName the company originating the entries in the batch

	// CompanyDiscretionaryData allows Originators and/or ODFIs to include codes (one or more),
	// of significance only to them, to enable specialized handling of all
	// subsequent entries in that batch. There will be no standardized
	// interpretation for the value of the field. This field must be returned
	// intact on any return entry.

	// CompanyIdentification The 9 digit FEIN number (proceeded by a predetermined
	// alpha or numeric character) of the entity in the company name field

	// StandardEntryClassCode
	// Identifies the payment type (product) found within an ACH batch-using a 3-character code.
	// The SEC Code pertains to all items within batch.
	// Determines format of the detail records.
	// Determines addenda records (required or optional PLUS one or up to 9,999 records).
	// Determines rules to follow (return time frames).
	// Some SEC codes require specific data in predetermined fields within the ACH record

	// CompanyEntryDescription A description of the entries contained in the batch
	//
	//The Originator establishes the value of this field to provide a
	// description of the purpose of the entry to be displayed back to
	// the receive For example, "GAS BILL," "REG. SALARY," "INS. PREM,"
	// "SOC. SEC.," "DTC," "TRADE PAY," "PURCHASE," etc.
	//
	// This field must contain the word "REVERSAL" (left justified) when the
	// batch contains reversing entries.
	//
	// This field must contain the word "RECLAIM" (left justified) when the
	// batch contains reclamation entries.
	//
	// This field must contain the word "NONSETTLED" (left justified) when the
	// batch contains entries which could not settle.
	CompanyEntryDescription string `json:"companyEntryDescription,omitempty"`

	// CompanyDescriptiveDate currently, the Rules provide that the “Originator establishes this field as the date it
	// would like to see displayed to the Receiver for descriptive purposes.” NACHA recommends that, as desired,
	// the content of this field be formatted using the convention “SDHHMM”, where the “SD” in positions 64- 65 denotes
	// the intent for same-day settlement, and the hours and minutes in positions 66-69 denote the desired settlement
	// time using a 24-hour clock. When electing to use this convention, the ODFI would validate that the field
	// contains either.
	//
	// ODFIs at their discretion may require their Originators to further show intent for
	// same-day settlement using an optional, yet standardized, same-day indicator in the Company Descriptive Date
	// field. The Company Descriptive Date field (5 record, field 8) is an optional field with 6 positions available
	// (positions 64-69).

	// EffectiveEntryDate the date on which the entries are to settle. Format: YYMMDD (Y=Year, M=Month, D=Day)
	EffectiveEntryDate string `json:"effectiveEntryDate,omitempty"`

	// SettlementDate Leave blank, this field is inserted by the ACH operator

	// OriginatorStatusCode refers to the ODFI initiating the Entry.
	// 0 ADV File prepared by an ACH Operator.
	// 1 This code identifies the Originator as a depository financial institution.
	// 2 This code identifies the Originator as a Federal Government entity or agency.

	//ODFIIdentification First 8 digits of the originating DFI transit routing number

	// BatchNumber is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by
	// record.

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
