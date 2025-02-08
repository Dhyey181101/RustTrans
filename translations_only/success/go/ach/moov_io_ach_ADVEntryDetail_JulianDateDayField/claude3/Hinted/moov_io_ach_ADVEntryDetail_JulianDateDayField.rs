
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i64, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = (max as usize) - s.len();
            let pad = unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    map.get(&m).cloned().unwrap_or_else(|| "0".repeat(m))
                } else {
                    let map = moov_io_ach_populate_map(94, "0");
                    MOOV_IO_ACH_STRING_ZEROS = Some(map);
                    MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).cloned().unwrap()
                }
            };
            pad + &s
        }
    }
}

struct MoovIoAchAdvEntryDetail {
    julian_day: i64,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.julian_day, 3)
    }
}
