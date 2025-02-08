
use std::collections::HashMap;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> Box<HashMap<i32, String>> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    Box::new(out)
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
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as i32;
        let pad = MOOV_IO_ACH_STRING_ZEROS
            .get(&m)
            .unwrap()
            .clone();
        format!("{}{}", pad, s)
    }
}

static MOOV_IO_ACH_STRING_ZEROS: once_cell::sync::Lazy<Box<HashMap<i32, String>>> =
    once_cell::sync::Lazy::new(|| moov_io_ach_populate_map(94, "0"));

fn main() {}
