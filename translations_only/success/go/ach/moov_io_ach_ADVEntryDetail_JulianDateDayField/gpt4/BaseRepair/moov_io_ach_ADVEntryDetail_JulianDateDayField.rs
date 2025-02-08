
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: Mutex<HashMap<i32, String>> = Mutex::new(moov_io_ach_populate_map(94, "0"));
}

struct MoovIoAchADVEntryDetail {
    julian_day: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchADVEntryDetail {
    fn julian_date_day_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.julian_day, 3)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            let pad = MOOV_IO_ACH_STRING_ZEROS.lock().unwrap().get(&m).unwrap().clone();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

fn main() {
}
