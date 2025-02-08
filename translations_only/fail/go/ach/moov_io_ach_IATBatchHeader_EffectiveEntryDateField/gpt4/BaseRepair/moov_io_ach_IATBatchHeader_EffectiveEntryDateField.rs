
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATBatchHeader {
    effective_entry_date: String,
}

struct MoovIoAchConverters;

impl MoovIoAchIATBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.effective_entry_date, 6)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap()
                    .clone()
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
