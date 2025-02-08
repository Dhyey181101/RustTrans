
use std::str::FromStr;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_SAVINGS_CREDIT: usize = 32;
const MOOV_IO_ACH_GLZERO_DOLLAR_REMITTANCE_CREDIT: usize = 44;
const MOOV_IO_ACH_LOANZERO_DOLLAR_REMITTANCE_CREDIT: usize = 54;
const MOOV_IO_ACH_CREDIT_SUMMARY: usize = 87;

impl MoovIoAchBatchControl {
    fn parse(&mut self, record: &str) {
        if record.chars().count() != MOOV_IO_ACH_RECORD_LENGTH {
            return;
        }

        // 1-1 Always "8"
        // 2-4 This is the same as the "Service code" field in previous Batch Header Record
        self.service_class_code = self.parse_num_field(&record[1..4]);
        // 5-10 Total number of Entry Detail Record in the batch
        self.entry_addenda_count = self.parse_num_field(&record[4..10]);
        // 11-20 Total of all positions 4-11 on each Entry Detail Record in the batch. This is essentially the sum of all the RDFI routing numbers in the batch.
        // If the sum exceeds 10 digits (because you have lots of Entry Detail Records), lop off the most significant digits of the sum until there are only 10
        self.entry_hash = self.parse_num_field(&record[10..20]);
        // 21-32 Number of cents of debit entries within the batch
        self.total_debit_entry_dollar_amount = self.parse_num_field(&record[20..32]);
        // 33-44 Number of cents of credit entries within the batch
        self.total_credit_entry_dollar_amount = self.parse_num_field(&record[32..44]);
        // 45-54 This is the same as the "Company identification" field in previous Batch Header Record
        self.company_identification = self.parse_string_field_with_opts(&record[44..54], &self.validate_opts);
        // 55-73 Seems to always be blank
        self.message_authentication_code = self.parse_string_field_with_opts(&record[54..73], &self.validate_opts);
        // 74-79 Always blank (just fill with spaces)
        // 80-87 This is the same as the "ODFI identification" field in previous Batch Header Record
        self.odfi_identification = self.parse_string_field_with_opts(&record[79..87], &self.validate_opts);
        // 88-94 This is the same as the "Batch number" field in previous Batch Header Record
        self.batch_number = self.parse_num_field(&record[87..94]);
    }

    fn parse_num_field(&self, r: &str) -> i32 {
        i32::from_str(&r.trim()).unwrap_or(0)
    }

    fn parse_string_field_with_opts(&self, r: &str, opts: &Box<MoovIoAchValidateOpts>) -> String {
        if opts.preserve_spaces {
            r.to_string()
        } else {
            self.parse_string_field(r)
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

#[derive(Default)]
struct MoovIoAchBatchControl {
    // ServiceClassCode ACH Mixed Debits and Credits '200'
    // ACH Credits Only '220'
    // ACH Debits Only '225'
    // Constants: MixedCreditsAnDebits (220), CReditsOnly 9220), DebitsOnly (225)
    // Same as 'ServiceClassCode' in BatchHeaderRecord
    service_class_code: i32,
    // EntryAddendaCount is a tally of each Entry Detail Record and each Addenda
    // Record processed, within either the batch or file as appropriate.
    entry_addenda_count: i32,
    // validate the Receiving DFI Identification in each Entry Detail Record is hashed
    // to provide a check against inadvertent alteration of data contents due
    // to hardware failure or program error
    //
    // In this context the Entry Hash is the sum of the corresponding fields in the
    // Entry Detail Records on the file.
    entry_hash: i32,
    // TotalDebitEntryDollarAmount Contains accumulated Entry debit totals within the batch.
    total_debit_entry_dollar_amount: i32,
    // TotalCreditEntryDollarAmount Contains accumulated Entry credit totals within the batch.
    total_credit_entry_dollar_amount: i32,
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
    company_identification: String,
    // MessageAuthenticationCode the MAC is an eight character code derived from a special key used in
    // conjunction with the DES algorithm. The purpose of the MAC is to
    // validate the authenticity of ACH entries. The DES algorithm and key
    // message standards must be in accordance with standards adopted by the
    // American National Standards Institute. The remaining eleven characters
    // of this field are blank.
    message_authentication_code: String,
    // ODFIIdentification the routing number is used to identify the DFI originating entries within a given branch.
    odfi_identification: String,
    // BatchNumber this number is assigned in ascending sequence to each batch by the ODFI
    // or its Sending Point in a given file of entries. Since the batch number
    // in the Batch Header Record and the Batch Control Record is the same,
    // the ascending sequence number should be assigned by batch and not by record.
    batch_number: i32,

    validate_opts: Box<MoovIoAchValidateOpts>,
}

#[derive(Default)]
struct MoovIoAchValidateOpts {
    // PreserveSpaces keeps the spacing before and after values that normally have spaces trimmed during parsing.
    preserve_spaces: bool,
}
