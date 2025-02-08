
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
    // Other fields omitted
}

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            " ".repeat(10)
        } else {
            let immediate_origin = self.immediate_origin.trim().to_string();
            if self.validate_opts.as_ref().map_or(false, |v| v.bypass_origin_validation)
                && immediate_origin.len() == 10
            {
                immediate_origin
            } else {
                [" ".to_string(), MoovIoAchConverters::string_field(&immediate_origin, 9)].concat()
            }
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + s
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

