

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let len = s.len();
        if len > max {
            s[len - max..].to_string()
        } else {
            let m = max - len;
            let pad = MoovIoAchAddenda18::get_zeros(m);
            pad + &s
        }
    }
}

struct MoovIoAchAddenda18 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda18 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn get_zeros(n: usize) -> String {
        let zeros = &ZEROS[..n];
        str::from_utf8(zeros).unwrap().to_string()
    }
}

impl fmt::Display for MoovIoAchAddenda18 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 18, ForeignCorrespondentBankName: ?, \
             ForeignCorrespondentBankIdentificationNumberQualifier: ?, \
             ForeignCorrespondentBankIdentificationNumber: ?, \
             ForeignCorrespondentBankBranchCountryCode: ?, \
             SequenceNumber: ?, EntryDetailSequenceNumber: {}, \
             EntryDetailSequenceNumberField: {}",
            self.entry_detail_sequence_number,
            self.entry_detail_sequence_number_field()
        )
    }
}

fn main() {
    let addenda18 = MoovIoAchAddenda18 {
        entry_detail_sequence_number: 123,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", addenda18);
}

