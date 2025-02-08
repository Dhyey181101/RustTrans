package test

import (
	"regexp"
	"strings"
	"time"
	"unicode/utf8"
)

var (
	moov_io_ach_hhmmRegex = regexp.MustCompile(`^([0-2]{1}[\d]{1}[0-5]{1}\d{1})$`)
)

const (
	moov_io_ach_RecordLength                      = 94
	moov_io_ach_CheckingPrenoteCredit             = 23
	moov_io_ach_CheckingZeroDollarRemittanceDebit = 29
	moov_io_ach_correctedDataCharLength           = 29
	moov_io_ach_SavingsPrenoteCredit              = 33
	moov_io_ach_SavingsZeroDollarRemittanceCredit = 34
	moov_io_ach_fileHeaderPos                     = "1"
	moov_io_ach_DebitForDebitsRejectedBatches     = 86
	moov_io_ach_GLPrenoteDebit                    = 48
)

func (fh *moov_io_ach_FileHeader) Parse(record string) {
	if utf8.RuneCountInString(record) != 94 {
		return
	}
	runes := []rune(record)

	// (character position 1-1) Always "1"
	// (2-3) Always "01"
	fh.priorityCode = "01"
	// (4-13) A blank space followed by your ODFI's routing number. For example: " 121140399"
	fh.ImmediateDestination = moov_io_ach_trimRoutingNumberLeadingZero(fh.parseStringField(string(runes[3:13])))
	// (14-23) A 10-digit number assigned to you by the ODFI once they approve you to originate ACH files through them
	fh.ImmediateOrigin = moov_io_ach_trimRoutingNumberLeadingZero(fh.parseStringField(string(runes[13:23])))
	// 24-29 Today's date in YYMMDD format
	// must be after today's date.
	fh.FileCreationDate = fh.validateSimpleDate(string(runes[23:29]))
	// 30-33 The current time in HHmm format
	fh.FileCreationTime = fh.validateSimpleTime(string(runes[29:33]))
	// 35-37 Always "A"
	fh.FileIDModifier = string(runes[33:34])
	// 35-37 always "094"
	fh.recordSize = "094"
	// 38-39 always "10"
	fh.blockingFactor = "10"
	// 40 always "1"
	fh.formatCode = "1"
	// 41-63 The name of the ODFI. example "SILICON VALLEY BANK    "
	fh.ImmediateDestinationName = fh.parseStringFieldWithOpts(string(runes[40:63]), fh.validateOpts)
	// 64-86 ACH operator or sending point that is sending the file
	fh.ImmediateOriginName = fh.parseStringFieldWithOpts(string(runes[63:86]), fh.validateOpts)
	// 87-94 Optional field that may be used to describe the ACH file for internal accounting purposes
	fh.ReferenceCode = fh.parseStringFieldWithOpts(string(runes[86:94]), fh.validateOpts)
}

func (c *moov_io_ach_converters) parseStringField(r string) (s string) {
	s = strings.TrimSpace(r)
	return s
}

func moov_io_ach_trimRoutingNumberLeadingZero(s string) string {
	if utf8.RuneCountInString(s) == 10 && s[0] == '0' && s != "0000000000" {
		// trim off a leading 0 as ImmediateOriginField or ImmediateDestinationField will pad it back
		return strings.TrimSpace(s[1:])
	}
	return strings.TrimSpace(s)
}

func (v *moov_io_ach_validator) validateSimpleDate(s string) string {
	_, err := time.Parse("060102", s) // YYMMDD
	if err != nil {
		return ""
	}
	return s
}

func (v *moov_io_ach_validator) validateSimpleTime(s string) string {
	if moov_io_ach_hhmmRegex.MatchString(s) {
		return s // successfully matched and validated
	}
	return ""
}

func (c *moov_io_ach_converters) parseStringFieldWithOpts(r string, opts *moov_io_ach_ValidateOpts) string {
	if opts != nil && opts.PreserveSpaces {
		return r
	} else {
		return c.parseStringField(r)
	}
}

type moov_io_ach_FileHeader struct {
	// ID is a client defined string used as a reference to this record.

	// PriorityCode consists of the numerals 01
	priorityCode string

	// ImmediateDestination contains the Routing Number of the ACH Operator or receiving
	// point to which the file is being sent. The ach file format specifies a 10 character
	// field begins with a blank space in the first position, followed by the four digit
	// Federal Reserve Routing Symbol, the four digit ABA Institution Identifier, and the Check
	// Digit (bTTTTAAAAC). ImmediateDestinationField will append the blank space to the
	// routing number.
	ImmediateDestination string `json:"immediateDestination"`

	// ImmediateOrigin contains the Routing Number of the ACH Operator or sending
	// point that is sending the file. The ach file format specifies a 10 character
	// field begins with a blank space in the first position, followed by the four digit
	// Federal Reserve Routing Symbol, the four digit ABA Institution Identifier, and the Check
	// Digit (bTTTTAAAAC). ImmediateOriginField will append the blank space to the
	// routing number.
	ImmediateOrigin string `json:"immediateOrigin"`

	// FileCreationDate is the date on which the file is prepared by an ODFI (ACH input files)
	// or the date (exchange date) on which a file is transmitted from ACH Operator
	// to ACH Operator, or from ACH Operator to RDFIs (ACH output files).
	//
	// The format is: YYMMDD. Y=Year, M=Month, D=Day
	FileCreationDate string `json:"fileCreationDate"`

	// FileCreationTime is the system time when the ACH file was created.
	//
	// The format is: HHmm. H=Hour, m=Minute
	FileCreationTime string `json:"fileCreationTime"`

	// This field should start at zero and increment by 1 (up to 9) and then go to
	// letters starting at A through Z for each subsequent file that is created for
	// a single system date. (34-34) 1 numeric 0-9 or uppercase alpha A-Z.
	// I have yet to see this ID not A
	FileIDModifier string `json:"fileIDModifier,omitempty"`

	// RecordSize indicates the number of characters contained in each
	// record. At this time, the value "094" must be used.
	recordSize string

	// BlockingFactor defines the number of physical records within a block
	// (a block is 940 characters). For all files moving between a DFI and an ACH
	// Operator (either way), the value "10" must be used. If the number of records
	// within the file is not a multiple of ten, the remainder of the block must
	// be nine-filled.
	blockingFactor string

	// FormatCode a code to allow for future format variations. As
	// currently defined, this field will contain a value of "1".
	formatCode string

	// ImmediateDestinationName us the name of the ACH or receiving point for which that
	// file is destined. Name corresponding to the ImmediateDestination
	ImmediateDestinationName string `json:"immediateDestinationName"`

	// ImmediateOriginName is the name of the ACH operator or sending point that is
	// sending the file. Name corresponding to the ImmediateOrigin
	ImmediateOriginName string `json:"immediateOriginName"`

	// ReferenceCode is reserved for information pertinent to the Originator.
	ReferenceCode string `json:"referenceCode,omitempty"`
	// validator is composed for data validation
	moov_io_ach_validator
	// converters is composed for ACH to GoLang Converters
	moov_io_ach_converters

	validateOpts *moov_io_ach_ValidateOpts
}

type moov_io_ach_converters struct{}

type moov_io_ach_validator struct{}

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
