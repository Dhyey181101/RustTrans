

use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_get_string_zeros() -> &'static Box<HashMap<usize, String>> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchFileControl {
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 12)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s[(l - max as usize)..].to_owned()
        } else {
            let m = (max as usize) - l;
            let pad = moov_io_ach_get_string_zeros().get(&m).unwrap().to_owned();
            pad + &s
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> Box<MoovIoAchConverters> {
        Box::new(MoovIoAchConverters)
    }
}

