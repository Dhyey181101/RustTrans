package test

import (
	"strconv"
	"strings"
)

var (
	moov_io_ach_stringZeros map[int]string = moov_io_ach_populateMap(94, "0")
)

const ()

func (fc *moov_io_ach_ADVFileControl) TotalDebitEntryDollarAmountInFileField() string {
	return fc.numericField(fc.TotalDebitEntryDollarAmountInFile, 20)
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

type moov_io_ach_ADVFileControl struct {
	// ID is a client defined string used as a reference to this record.

	// BatchCount total number of batches (i.e., '5' records) in the file

	// BlockCount total number of records in the file (include all headers and trailer) divided
	// by 10 (This number must be evenly divisible by 10. If not, additional records consisting of all 9's are added to the file after the initial '9' record to fill out the block 10.)

	// EntryAddendaCount total detail and addenda records in the file

	// EntryHash calculated in the same manner as the batch has total but includes total from entire file

	// TotalDebitEntryDollarAmountInFile contains accumulated Batch debit totals within the file.
	TotalDebitEntryDollarAmountInFile int `json:"totalDebit"`

	// TotalCreditEntryDollarAmountInFile contains accumulated Batch credit totals within the file.

	// validator is composed for data validation

	// converters is composed for ACH to golang Converters
	moov_io_ach_converters
}

type moov_io_ach_converters struct{}
