

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

const ZEROS: &str = "0";

struct MoovIoAchFileHeader {
    immediate_origin: String,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            " ".repeat(10)
        } else {
            let mut origin = self.immediate_origin.trim();
            if let Some(ref validate_opts) = self.validate_opts {
                if validate_opts.bypass_origin_validation && origin.len() == 10 {
                    return origin.to_string();
                }
            }
            format!(" {}" , self.string_field(origin, 9))
        }
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..(max as usize)].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                Some(p) => p.clone(),
                None => ZEROS.repeat(m),
            };
            pad + s
        }
    }
}

impl MoovIoAchConverters {
    // ... other methods ...
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
    // ... other fields ...
}

impl Default for MoovIoAchValidateOpts {
    fn default() -> Self {
        MoovIoAchValidateOpts {
            bypass_origin_validation: false,
            // ... other fields initialized to default values ...
        }
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::new();
    for i in 0..100 {
        out.insert(i, ZEROS.repeat(i));
    }
    out
});

impl fmt::Display for MoovIoAchFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.immediate_origin_field())
    }
}

