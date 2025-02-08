
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: MoovIoAchValidateOpts,
}

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let mut immediate_destination = self.immediate_destination.trim().to_string();
        if self.validate_opts.bypass_destination_validation && immediate_destination.len() == 10 {
            return immediate_destination;
        }
        return " ".to_string() + &MoovIoAchConverters::string_field(&immediate_destination, 9);
    }
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap().clone();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

