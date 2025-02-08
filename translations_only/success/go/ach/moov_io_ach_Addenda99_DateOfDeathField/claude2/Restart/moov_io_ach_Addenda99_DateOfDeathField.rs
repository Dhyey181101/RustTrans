

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99 {
    date_of_death: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99 {
    fn date_of_death_field(&self) -> String {
        if self.date_of_death.is_empty() {
            self.moov_io_ach_converters.alpha_field("".to_string(), 6)
        } else {
            self.moov_io_ach_converters.format_simple_date(self.date_of_death.clone())
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            if let Some(pad) = MOOV_IO_ACH_SPACEZEROS.get(&m) {
                format!("{}{}", s, pad)
            } else {
                format!("{}{}", s, " ".repeat(m as usize))
            }
        }
    }

    fn format_simple_date(&self, s: String) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s
        }
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            if let Some(pad) = MOOV_IO_ACH_STRINGZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                format!("{}{}", "0".repeat(m as usize), s)
            }
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

