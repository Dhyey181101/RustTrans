
use std::collections::HashMap;
use std::str::Chars;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

struct MoovIoAchAddenda99Contested {
    original_settlement_date: String,
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda99Contested {
    fn original_settlement_date_field(&self) -> String {
        self.converters.string_field(&self.original_settlement_date, 3)
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

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

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out: HashMap<i32, String> = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
