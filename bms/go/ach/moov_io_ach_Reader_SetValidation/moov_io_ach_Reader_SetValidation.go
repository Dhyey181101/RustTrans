package test

var ()

const ()

func (r *moov_io_ach_Reader) SetValidation(opts *moov_io_ach_ValidateOpts) {
	if r == nil || opts == nil {
		return
	}
	r.File.SetValidation(opts)
}

func (f *moov_io_ach_File) SetValidation(opts *moov_io_ach_ValidateOpts) {
	if f == nil {
		return
	}

	f.validateOpts = opts
	f.Header.SetValidation(opts)
}

func (fh *moov_io_ach_FileHeader) SetValidation(opts *moov_io_ach_ValidateOpts) {
	if fh == nil {
		return
	}
	fh.validateOpts = opts
}

type moov_io_ach_Reader struct {
	// file is ach.file model being built as r is parsed.
	File moov_io_ach_File

	// IATCurrentBatch is the current IATBatch entries being parsed

	// r handles the IO.Reader sent to be parser.

	// line is the current line being parsed from the input r

	// currentBatch is the current Batch entries being parsed

	// line number of the file being parsed

	// maxLines is the maximum number of lines to be consumed

	// recordName holds the current record name being parsed.

	// errors holds each error encountered when attempting to parse the file

	// skipBatchAccumulation is a flag to skip .AddBatch

}

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

	// AllowInvalidAmounts will skip verifying the Amount is valid for the TransactionCode and entry type.

}

type moov_io_ach_File struct {
	Header moov_io_ach_FileHeader `json:"fileHeader"`

	// NotificationOfChange (Notification of change) is a slice of references to BatchCOR in file.Batches

	// ReturnEntries is a slice of references to file.Batches that contain return entries

	validateOpts *moov_io_ach_ValidateOpts
}

type moov_io_ach_FileHeader struct {
	// ID is a client defined string used as a reference to this record.

	// PriorityCode consists of the numerals 01

	// ImmediateDestination contains the Routing Number of the ACH Operator or receiving
	// point to which the file is being sent. The ach file format specifies a 10 character
	// field begins with a blank space in the first position, followed by the four digit
	// Federal Reserve Routing Symbol, the four digit ABA Institution Identifier, and the Check
	// Digit (bTTTTAAAAC). ImmediateDestinationField will append the blank space to the
	// routing number.

	// ImmediateOrigin contains the Routing Number of the ACH Operator or sending
	// point that is sending the file. The ach file format specifies a 10 character
	// field begins with a blank space in the first position, followed by the four digit
	// Federal Reserve Routing Symbol, the four digit ABA Institution Identifier, and the Check
	// Digit (bTTTTAAAAC). ImmediateOriginField will append the blank space to the
	// routing number.

	// FileCreationDate is the date on which the file is prepared by an ODFI (ACH input files)
	// or the date (exchange date) on which a file is transmitted from ACH Operator
	// to ACH Operator, or from ACH Operator to RDFIs (ACH output files).
	//
	// The format is: YYMMDD. Y=Year, M=Month, D=Day

	// FileCreationTime is the system time when the ACH file was created.
	//
	// The format is: HHmm. H=Hour, m=Minute

	// This field should start at zero and increment by 1 (up to 9) and then go to
	// letters starting at A through Z for each subsequent file that is created for
	// a single system date. (34-34) 1 numeric 0-9 or uppercase alpha A-Z.
	// I have yet to see this ID not A

	// RecordSize indicates the number of characters contained in each
	// record. At this time, the value "094" must be used.

	// BlockingFactor defines the number of physical records within a block
	// (a block is 940 characters). For all files moving between a DFI and an ACH
	// Operator (either way), the value "10" must be used. If the number of records
	// within the file is not a multiple of ten, the remainder of the block must
	// be nine-filled.

	// FormatCode a code to allow for future format variations. As
	// currently defined, this field will contain a value of "1".

	// ImmediateDestinationName us the name of the ACH or receiving point for which that
	// file is destined. Name corresponding to the ImmediateDestination

	// ImmediateOriginName is the name of the ACH operator or sending point that is
	// sending the file. Name corresponding to the ImmediateOrigin

	// ReferenceCode is reserved for information pertinent to the Originator.

	// validator is composed for data validation

	// converters is composed for ACH to GoLang Converters

	validateOpts *moov_io_ach_ValidateOpts
}
