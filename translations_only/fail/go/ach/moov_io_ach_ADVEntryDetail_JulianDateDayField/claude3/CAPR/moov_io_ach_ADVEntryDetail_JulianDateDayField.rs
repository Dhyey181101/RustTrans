
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: usize, max: u32) -> String {
    let s = n.to_string();
    if s.len() > max as usize {
        String::from(&s[(s.len() - max as usize)..])
    } else {
        let m = (max as usize) - s.len();
        unsafe {
            if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    pad.to_owned() + &s
                } else {
                    "0".repeat(m) + &s
                }
            } else {
                let map = moov_io_ach_populate_map(94, "0");
                MOOV_IO_ACH_STRING_ZEROS = Some(map);
                moov_io_ach_numeric_field(n, max)
            }
        }
    }
}

struct MoovIoAchAdvEntryDetail {
    julian_day: usize,
}

impl MoovIoAchAdvEntryDetail {
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
