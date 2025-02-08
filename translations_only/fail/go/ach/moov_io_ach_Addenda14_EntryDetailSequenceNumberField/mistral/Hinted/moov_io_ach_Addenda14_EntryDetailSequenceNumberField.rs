

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() as usize - max as usize)..].to_string();
        }
        let m = max as i32 - s.len() as i32;
        match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            Some(pad) => format!("{}{}", pad, &s),
            None => "".to_string() + &s[..],
        }
    }
}

struct MoovIoAchAddenda14 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda14 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl fmt::Display for MoovIoAchAddenda14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: ?\tTypeCode: Addenda14 types code '14'\tName: ?\tDFI Qualifier: ?\tDFI: ?\tCountry Code: ?\tEntryDetailSequenceNumber: {}\t",
            self.entry_detail_sequence_number
        )
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, ("0".repeat(i as usize)).to_string());
    }
    out
});

fn main() {
    let addenda = MoovIoAchAddenda14 {
        entry_detail_sequence_number: 123456,
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", addenda);
}

