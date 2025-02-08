

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<str>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static HashMap<usize, Box<str>> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            String::from(&s[s.len() - max..])
        } else {
            let m = max - s.len();
            let pad = moov_io_ach_get_string_zeros().get(&m).unwrap().to_string();
            pad + &s
        }
    }
}

struct MoovIoAchAdvFileControl {
    total_debit_entry_dollar_amount_in_file: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvFileControl {
    fn total_debit_entry_dollar_amount_in_file_field(&self) -> String {
        self.moov_io_ach_converters
            .numeric_field(self.total_debit_entry_dollar_amount_in_file, 20)
    }
}

