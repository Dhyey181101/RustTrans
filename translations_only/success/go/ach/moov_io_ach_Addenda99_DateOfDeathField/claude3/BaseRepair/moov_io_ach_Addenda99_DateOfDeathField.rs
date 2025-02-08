
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchAddenda99 {
    date_of_death: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99 {
    fn date_of_death_field(&self) -> String {
        if self.date_of_death.is_empty() {
            self.moov_io_ach_converters.alpha_field("", 6)
        } else {
            self.moov_io_ach_converters.format_simple_date(&self.date_of_death)
        }
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = MOOV_IO_ACH_SPACE_ZEROS.get(&m) {
                s.to_string() + pad
            } else {
                s.to_string() + &" ".repeat(m)
            }
        }
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 6)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                pad.to_string() + s
            } else {
                "0".repeat(m) + s
            }
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}
