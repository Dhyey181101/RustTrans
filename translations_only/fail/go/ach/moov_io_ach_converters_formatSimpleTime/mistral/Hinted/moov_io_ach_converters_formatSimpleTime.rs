

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

struct MoovIoAchConverters {
    string_zeros: HashMap<usize, String>,
}

const MOOV_IO_ACH_ZERO: &str = "0";

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {
            string_zeros: moov_io_ach_populate_map(94, MOOV_IO_ACH_ZERO),
        }
    }

    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 4);
        }
        s.to_string()
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = if let Some(pad) = self.string_zeros.get(&m) {
            pad.clone()
        } else {
            MOOV_IO_ACH_ZERO.repeat(m)
        };

        pad + s
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "moov_io_ach_converters \n(string_zeros: {:?})",
            self.string_zeros
        )
    }
}

static MOOV_IO_ACH_ZERO_MAP: Lazy<HashMap<usize, String>> = Lazy::new(|| MoovIoAchConverters::new().string_zeros);

