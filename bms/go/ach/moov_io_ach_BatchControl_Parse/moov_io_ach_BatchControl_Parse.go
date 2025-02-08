package test

import (
	"strconv"
	"strings"
	"unicode/utf8"
)

var ()

const (
	moov_io_ach_RecordLength                   = 94
	moov_io_ach_SavingsCredit                  = 32
	moov_io_ach_GLZeroDollarRemittanceCredit   = 44
	moov_io_ach_LoanZeroDollarRemittanceCredit = 54
	moov_io_ach_CreditSummary                  = 87
)

func (bc *moov_io_ach_BatchControl) Parse(record string) {
	if utf8.RuneCountInString(record) != 94 {
		return
	}

	// 1-1 Always "8"
	// 2-4 This is the same as the "Service code" field in previous Batch Header Record
	bc.ServiceClassCode = bc.parseNumField(record[1:4])
	// 5-10 Total number of Entry Detail Record in the batch
	bc.EntryAddendaCount = bc.parseNumField(record[4:10])
	// 11-20 Total of all positions 4-11 on each Entry Detail Record in the batch. This is essentially the sum of all the RDFI routing numbers in the batch.
	// If the sum exceeds 10 digits (because you have lots of Entry Detail Records), lop off the most significant digits of the sum until there are only 10
	bc.EntryHash = bc.parseNumField(record[10:20])
	// 21-32 Number of cents of debit entries within the batch
	bc.TotalDebitEntryDollarAmount = bc.parseNumField(record[20:32])
	// 33-44 Number of cents of credit entries within the batch
	bc.TotalCreditEntryDollarAmount = bc.parseNumField(record[32:44])
	// 45-54 This is the same as the "Company identification" field in previous Batch Header Record
	bc.CompanyIdentification = bc.parseStringFieldWithOpts(record[44:54], bc.validateOpts)
	// 55-73 Seems to always be blank
	bc.MessageAuthenticationCode = bc.parseStringFieldWithOpts(record[54:73], bc.validateOpts)
	// 74-79 Always blank (just fill with spaces)
	// 80-87 This is the same as the "ODFI identification" field in previous Batch Header Record
	bc.ODFIIdentification = bc.parseStringFieldWithOpts(record[79:87], bc.validateOpts)
	// 88-94 This is the same as the "Batch number" field in previous Batch Header Record
	bc.BatchNumber = bc.parseNumField(record[87:94])
}

func (c *moov_io_ach_converters) parseNumField(r string) (s int) {
	s, _ = strconv.Atoi(strings.TrimSpace(r))
	return s
}

func (c *moov_io_ach_converters) parseStringFieldWithOpts(r string, opts *moov_io_ach_ValidateOpts) string {
	if opts != nil && opts.PreserveSpaces {
		return r
	} else {
		return c.parseStringField(r)
	}
}

func (c *moov_io_ach_converters) parseStringField(r string) (s string) {
	s = strings.TrimSpace(r)
	return s
}

type moov_io_ach_BatchControl struct {
	// ID is a client defined string used as a reference to this record.

	// ServiceClassCode ACH Mixed Debits and Credits '200'
	// ACH Credits Only '220'
	// ACH Debits Only '225'
	// Constants: MixedCreditsAnDebits (220), CReditsOnly 9220), DebitsOnly (225)
	// Same as 'ServiceClassCode' in BatchHeaderRecord
	ServiceClassCode int `json:"serviceClassCode"`
	// EntryAddendaCount is a tally of each Entry Detail Record and each Addenda
	// Record processed, within either the batch or file as appropriate.
	EntryAddendaCount int `json:"entryAddendaCount"`
	// validate the Receiving DFI Identification in each Entry Detail Record is hashed
	// to provide a check against inadvertent alteration of data contents due
	// to hardware failure or program error
	//
	// In this context the Entry Hash is the sum of the corresponding fields in the
	// Entry Detail Records on the file.
	EntryHash int `json:"entryHash"`
	// TotalDebitEntryDollarAmount Contains accumulated Entry debit totals within the batch.
	TotalDebitEntryDollarAmount int `json:"totalDebit"`
	// TotalCreditEntryDollarAmount Contains accumulated Entry credit totals within the batch.
	TotalCreditEntryDollarAmount int `json:"totalCredit"`
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
	CompanyIdentification string `json:"companyIdentification"`
	// MessageAuthenticationCode the MAC is an eight character code derived from a special key used in
	// conjunction with the DES algorithm. The purpose of the MAC is to
	// validate the authenticity of ACH entries. The DES algorithm and key
	// message standards must be in accordance with standards adopted by the
	// American National Standards Institute. The remaining eleven characters
	// of this field are blank.
	MessageAuthenticationCode string `json:"messageAuthentication,omitempty"`
	// ODFIIdentification the routing number is used to identify the DFI originating entries within a given branch.
	ODFIIdentification string `json:"ODFIIdentification"`
	// BatchNumber this number is assigned in ascending sequence to each batch by the ODFI
	// or its Sending Point in a given file of entries. Since the batch number
	// in the Batch Header Record and the Batch Control Record is the same,
	// the ascending sequence number should be assigned by batch and not by record.
	BatchNumber int `json:"batchNumber"`
	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters

	validateOpts *moov_io_ach_ValidateOpts
}

type moov_io_ach_converters struct{}

type moov_io_ach_ValidateOpts struct {
	// SkipAll will disable all validation checks of a File. It has no effect when set on records.

	// RequireABAOrigin can be set to enable routing number validation
	// over the ImmediateOrigin file header field.

	// BypassOriginValidation can be set to skip validation for the
	// ImmediateOrigin file header field.
	//
	// This also allows for custom TraceNumbers which aren't prefixed with
	// a routing number as required by the NACHA specification.

	// BypassDestinationValidation can be set to skip validation for the
	// ImmediateDestination file header field.
	//
	// This also allows for custom TraceNumbers which aren't prefixed with
	// a routing number as required by the NACHA specification.

	// CheckTransactionCode allows for custom validation of TransactionCode values
	//
	// Note: Functions cannot be serialized into/from JSON, so this check cannot be used from config files.

	// CustomTraceNumbers disables Nacha specified checks of TraceNumbers:
	// - Ascending order of trace numbers within batches
	// - Trace numbers beginning with their ODFI's routing number
	// - AddendaRecordIndicator is set correctly

	// AllowZeroBatches allows the file to have zero batches

	// AllowMissingFileHeader allows a file to be read without a FileHeader record.

	// AllowMissingFileControl allows a file to be read without a FileControl record.

	// BypassCompanyIdentificationMatch allows batches in which the Company Identification field
	// in the batch header and control do not match.

	// CustomReturnCodes can be set to skip validation for the Return Code field in an Addenda99
	// This allows for non-standard/deprecated return codes (e.g. R97)

	// UnequalServiceClassCode skips equality checks for the ServiceClassCode in each pair of BatchHeader
	// and BatchControl records.

	// AllowUnorderedBatchNumebrs allows a file to be read with unordered batch numbers.

	// AllowInvalidCheckDigit allows the CheckDigit field in EntryDetail to differ from
	// the expected calculation

	// UnequalAddendaCounts skips checking that Addenda Count fields match their expected and computed values.

	// PreserveSpaces keeps the spacing before and after values that normally have spaces trimmed during parsing.
	PreserveSpaces bool `json:"preserveSpaces"`

	// AllowInvalidAmounts will skip verifying the Amount is valid for the TransactionCode and entry type.

}
