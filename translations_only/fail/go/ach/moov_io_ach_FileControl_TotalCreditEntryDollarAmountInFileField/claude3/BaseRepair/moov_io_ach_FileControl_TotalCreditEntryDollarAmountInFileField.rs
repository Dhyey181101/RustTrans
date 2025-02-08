
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, Box<String>>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, Box<String>>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static Box<HashMap<usize, Box<String>>> {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS
            .get_or_insert_with(|| moov_io_ach_populate_map(94, "0"))
    }
}

struct MoovIoAchFileControl {
    total_credit_entry_dollar_amount_in_file: usize,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 12)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            let mut result = String::new();
            for c in s.chars().rev().take(max as usize) {
                result.push(c);
            }
            result
        } else {
            let m = max as usize - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).unwrap_or(&Box::new("".to_string())).to_string();
            pad + &s
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Box<Self> {
        Box::new(MoovIoAchConverters {})
    }
}
