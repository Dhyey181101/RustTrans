

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchAddenda14::get_zeros(m as usize);
            return pad + &s;
        }
    }
}

struct MoovIoAchAddenda14 {
    entry_detail_sequence_number: i32,
    _converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda14 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        self._converters.numeric_field(n, max)
    }

    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchAddenda14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?, TypeCode: 14, ReceiverBankName: ?, DFIIdentificationQualifier: ?, \
             ReceivingDFIIdentification: ?, ReceivingDFIBranchCountryCode: ?, \
             EntryDetailSequenceNumber: {}, validator: ?",
            self.entry_detail_sequence_number
        )
    }
}

