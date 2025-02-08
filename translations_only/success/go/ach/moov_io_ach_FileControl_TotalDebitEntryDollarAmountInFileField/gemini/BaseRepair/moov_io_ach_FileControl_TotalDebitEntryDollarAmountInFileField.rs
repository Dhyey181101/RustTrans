
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchFileControl {
    pub total_debit_entry_dollar_amount_in_file: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchFileControl {
    pub fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_debit_entry_dollar_amount_in_file, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

impl MoovIoAchConverters {}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}
