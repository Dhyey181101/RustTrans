
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| moov_io_ach_populate_map(94, Box::from("0")));

fn moov_io_ach_populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

struct MoovIoAchAdvEntryDetail {
    julian_day: usize,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> Box<str> {
        self.converters.numeric_field(self.julian_day, 3)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> Box<str> {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            let mut chars = s.chars().rev().collect::<Vec<_>>();
            chars.truncate(max as usize);
            Box::from(chars.into_iter().rev().collect::<String>())
        } else {
            let m = max as usize - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&Box::from("".to_string())).to_string();
            Box::from(pad + &s)
        }
    }
}
