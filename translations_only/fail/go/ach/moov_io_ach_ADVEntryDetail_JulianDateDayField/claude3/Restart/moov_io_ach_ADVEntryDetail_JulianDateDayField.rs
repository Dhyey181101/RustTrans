
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = Box::new(HashMap::with_capacity(max));
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: usize, max: u32) -> String {
    let s = n.to_string();
    if s.len() > max as usize {
        return s[s.len() - max as usize..].to_string();
    } else {
        let m = (max as usize) - s.len();
        unsafe {
            if let Some(map) = MOOV_IO_ACH_STRING_ZEROS.as_ref() {
                if let Some(pad) = map.get(&m) {
                    return pad.to_string() + &s;
                }
            } else {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
        }
        "0".repeat(m) + &s
    }
}

struct MoovIoAchAdventryDetail {
    julian_day: usize,
}

impl MoovIoAchAdventryDetail {
    fn julian_date_day_field(&self) -> String {
        moov_io_ach_numeric_field(self.julian_day, 3)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        moov_io_ach_numeric_field(n, max)
    }
}
