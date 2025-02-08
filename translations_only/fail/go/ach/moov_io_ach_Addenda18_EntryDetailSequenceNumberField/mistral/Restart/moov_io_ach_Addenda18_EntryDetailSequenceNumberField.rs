

use std::collections::HashMap;
use std::fmt;
use std::string::String;

const ZERO: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as usize) - (max as usize)..].to_string();
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = MoovIoAchAddenda18::get_pad_string(m);
            return format!("{}{}", pad, &s);
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

    fn get_pad_string(n: i32) -> String {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        if let Some(v) = out.get(&(n as usize)) {
            return v.clone();
        }
        "0".repeat(n as usize)
    }
}

impl fmt::Display for MoovIoAchAddenda18 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 18, ForeignCorrespondentBankName: ?, ForeignCorrespondentBankIdentificationNumberQualifier: ?, ForeignCorrespondentBankIdentificationNumber: ?, ForeignCorrespondentBankBranchCountryCode: ?, SequenceNumber: ?, EntryDetailSequenceNumber: {}",
            self.entry_detail_sequence_number
        )
    }
}

