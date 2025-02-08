
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, Box<String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, Box<String>> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i as usize)));
    }
    out
}

struct MoovIoAchADVBatchControl {
    total_credit_entry_dollar_amount: i32,
}

impl MoovIoAchADVBatchControl {
    fn total_credit_entry_dollar_amount_field(&self) -> String {
        self.numeric_field(self.total_credit_entry_dollar_amount, 20)
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
                    .map_or_else(|| Box::new("0".repeat(m as usize)), |s| s.clone())
            };
            format!("{}{}", pad, s)
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
