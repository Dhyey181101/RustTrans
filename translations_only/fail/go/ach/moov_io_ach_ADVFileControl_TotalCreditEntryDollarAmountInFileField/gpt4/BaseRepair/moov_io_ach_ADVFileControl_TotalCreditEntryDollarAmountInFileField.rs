
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchADVFileControl {
    total_credit_entry_dollar_amount_in_file: i32,
}

impl MoovIoAchADVFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount_in_file, 20)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = (max - l) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap()
                    .clone()
            };
            pad + &s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
