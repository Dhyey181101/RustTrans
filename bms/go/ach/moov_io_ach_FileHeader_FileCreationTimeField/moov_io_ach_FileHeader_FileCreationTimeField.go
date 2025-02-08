package test

import (
	"strings"
	"time"
	"unicode/utf8"

	"github.com/moov-io/base"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (fh *moov_io_ach_FileHeader) FileCreationTimeField() string {
	switch utf8.RuneCountInString(fh.FileCreationTime) {
	case 0:
		return time.Now().Format("1504")
	case 4:
		return fh.formatSimpleTime(fh.FileCreationTime) // HHmm
	}
	t, err := time.Parse(base.ISO8601Format, fh.FileCreationTime)
	if err != nil {
		return ""
	}
	return t.Format("1504") // HHmm
}

func (c *moov_io_ach_converters) formatSimpleTime(s string) string {
	if s == "" {
		return c.stringField(s, 4)
	}
	return s
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
	FileCreationTime string `json:"fileCreationTime"`

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
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
