
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
            return pad.to_owned() + s;
        }
    }
    "0".repeat(m) + s
}

struct MoovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Box<MoovIoAchValidateOpts>,
}

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        let immediate_origin = self.immediate_origin.trim().to_owned();
        if self.validate_opts.bypass_origin_validation && immediate_origin.len() == 10 {
            return immediate_origin;
        }
        " ".to_owned() + &moov_io_ach_string_field(&immediate_origin, 9)
    }
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

struct MoovIoAchConverters {}
