
use std::str::FromStr;

#[derive(Debug)]
pub struct BatchControl {
    pub service_class_code: i32,
    pub entry_addenda_count: i32,
    pub entry_hash: i32,
    pub total_debit_entry_dollar_amount: i32,
    pub total_credit_entry_dollar_amount: i32,
    pub company_identification: String,
    pub message_authentication_code: String,
    pub odfi_identification: String,
    pub batch_number: i32,
    pub validate_opts: Option<ValidateOpts>,
}

impl BatchControl {
    pub fn parse(&mut self, record: &str) {
        if record.len() != 94 {
            return;
        }

        // 1-1 Always "8"
        // 2-4 This is the same as the "Service code" field in previous Batch Header Record
        self.service_class_code = i32::from_str(&record[1..4]).unwrap();
        // 5-10 Total number of Entry Detail Record in the batch
        self.entry_addenda_count = i32::from_str(&record[4..10]).unwrap();
        // 11-20 Total of all positions 4-11 on each Entry Detail Record in the batch. This is essentially the sum of all the RDFI routing numbers in the batch.
        // If the sum exceeds 10 digits (because you have lots of Entry Detail Records), lop off the most significant digits of the sum until there are only 10
        self.entry_hash = i32::from_str(&record[10..20]).unwrap();
        // 21-32 Number of cents of debit entries within the batch
        self.total_debit_entry_dollar_amount = i32::from_str(&record[20..32]).unwrap();
        // 33-44 Number of cents of credit entries within the batch
        self.total_credit_entry_dollar_amount = i32::from_str(&record[32..44]).unwrap();
        // 45-54 This is the same as the "Company identification" field in previous Batch Header Record
        self.company_identification = record[44..54].to_string();
        // 55-73 Seems to always be blank
        self.message_authentication_code = record[54..73].to_string();
        // 74-79 Always blank (just fill with spaces)
        // 80-87 This is the same as the "ODFI identification" field in previous Batch Header Record
        self.odfi_identification = record[79..87].to_string();
        // 88-94 This is the same as the "Batch number" field in previous Batch Header Record
        self.batch_number = i32::from_str(&record[87..94]).unwrap();
    }
}

#[derive(Debug)]
pub struct ValidateOpts {
    pub preserve_spaces: bool,
}

impl ValidateOpts {
    pub fn new(preserve_spaces: bool) -> Self {
        Self { preserve_spaces }
    }
}

#[derive(Debug)]
pub struct Converters {}

impl Converters {
    pub fn parse_num_field(&self, r: &str) -> i32 {
        i32::from_str(r.trim()).unwrap()
    }

    pub fn parse_string_field_with_opts(&self, r: &str, opts: &Option<ValidateOpts>) -> String {
        if let Some(opts) = opts {
            if opts.preserve_spaces {
                r.to_string()
            } else {
                self.parse_string_field(r)
            }
        } else {
            self.parse_string_field(r)
        }
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}
