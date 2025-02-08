

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        self.converters.string_field(&self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters {
    zeros: &'static [u8],
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            return s[..max as usize].to_string();
        }

        let m = (max - len) as usize;
        let pad = if let Some(pad) = self.zeros.get(m) {
            pad.to_string()
        } else {
            ZEROS[..m].iter().map(|c| *c as char).collect()
        };

        format!("{}{}", pad, s)
    }
}

impl fmt::Display for MoovIoAchConverters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MoovIoAchConverters")
    }
}

static SELF: MoovIoAchConverters = MoovIoAchConverters { zeros: ZEROS };

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, ("0".repeat(i)).to_string());
    }
    out
}

#[derive(Default)]
struct MoovIoAch {
    string_zeros: HashMap<usize, String>,
}

impl MoovIoAch {
    fn new() -> Self {
        let mut moov_io_ach = MoovIoAch {
            string_zeros: moov_io_ach_populate_map(94, "0"),
            ..Default::default()
        };
        moov_io_ach.string_zeros.shrink_to_fit();
        moov_io_ach
    }
}

