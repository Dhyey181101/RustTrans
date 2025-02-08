
use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        let trimmed = self.immediate_origin.trim();
        if let Some(opts) = &self.validate_opts {
            if opts.bypass_origin_validation && trimmed.len() == 10 {
                return trimmed.to_string();
            }
        }
        format!(" {}", moov_io_ach_string_field(trimmed, 9))
    }
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let len = s.chars().count() as u32;
    if len > max {
        return s[..max as usize].to_string();
    }

    let m = (max - len) as usize;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    format!("{}{}", "0".repeat(m), s)
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

struct MoovIoAchConverters;
