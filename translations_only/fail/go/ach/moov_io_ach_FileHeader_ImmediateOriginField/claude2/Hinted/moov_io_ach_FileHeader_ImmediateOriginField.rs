
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0"));

struct MovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Option<Box<MovIoAchValidateOpts>>,
}

struct MovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

impl MovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        let mut immediate_origin = self.immediate_origin.trim().to_string();
        if let Some(opts) = self.validate_opts.as_ref() {
            if opts.bypass_origin_validation && immediate_origin.len() == 10 {
                return immediate_origin;
            }
        }
        " ".to_owned() + &string_field(&immediate_origin, 9)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as i32;
    if let Some(pad) = MOV_IO_ACH_STRING_ZEROS.get(&m) {
        return pad.to_string() + s;
    }

    "0".repeat(m as usize) + s
}

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

