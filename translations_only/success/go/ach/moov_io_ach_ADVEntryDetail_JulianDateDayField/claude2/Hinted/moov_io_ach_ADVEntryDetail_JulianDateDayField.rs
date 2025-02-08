
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    populate_map(94, "0".to_string())
});

struct MoovIoAchAdvEntryDetail {
    julian_day: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchAdvEntryDetail {
    fn julian_date_day_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.julian_day, 3)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len()-max as usize..].to_string()
        } else {
            let m = (max - s.len() as u32) as i32;
            match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                Some(pad) => format!("{}{}", pad, s),
                None => "0".repeat(m as usize) + &s,
            }
        }
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

