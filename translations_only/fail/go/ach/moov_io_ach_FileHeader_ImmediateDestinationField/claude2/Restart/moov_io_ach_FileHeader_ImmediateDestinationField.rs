
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

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
        if self.validate_opts
            .as_ref()
            .map(|v| v.bypass_destination_validation)
            .unwrap_or(false)
            && immediate_destination.len() == 10
        {
            return immediate_destination;
        }
        return " ".to_owned() + &string_field(&immediate_destination, 9);
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m)) + s
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

struct MoovIoAchConverters;

