
use once_cell::sync::Lazy;
use std::collections::HashMap;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let mut immediate_destination = self.immediate_destination.trim().to_string();
        if self.validate_opts
            .as_ref()
            .map(|v| v.bypass_destination_validation)
            .unwrap_or(false)
            && immediate_destination.len() == 10
        {
            return immediate_destination;
        }
        let mut result = " ".to_string();
        result.push_str(&MoovIoAchConverters::string_field(
            immediate_destination,
            9,
        ));
        result
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        MOOV_IO_ACH_STRING_ZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + &s
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
}

