
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Box<MoovIoAchValidateOpts>,
}

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            " ".repeat(10)
        } else {
            let mut immediate_origin = self.immediate_origin.trim().to_string();
            if self.validate_opts.bypass_origin_validation && immediate_origin.len() == 10 {
                immediate_origin
            } else {
                " ".to_owned() + &MoovIoAchConverters::string_field(&immediate_origin, 9)
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
            match MOOV_IO_ACH_STRINGZEROS.get(&m) {
                Some(pad) => pad.to_owned() + s,
                None => "0".repeat(m) + s,
            }
        }
    }
}

fn populate_map(max: i32, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

