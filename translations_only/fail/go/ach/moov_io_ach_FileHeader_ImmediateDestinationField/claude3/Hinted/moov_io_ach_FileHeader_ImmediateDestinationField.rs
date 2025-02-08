
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let immediate_destination = self.immediate_destination.trim().to_string();
        if let Some(validate_opts) = &self.validate_opts {
            if validate_opts.bypass_destination_validation && immediate_destination.len() == 10 {
                return immediate_destination;
            }
        }
        let mut result = " ".to_string();
        result.push_str(&self.string_field(&immediate_destination, 9));
        result
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(map) = &MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    return pad.to_string() + s;
                }
            }
        }

        "0".repeat(m) + s
    }
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
}

struct MoovIoAchConverters {}
