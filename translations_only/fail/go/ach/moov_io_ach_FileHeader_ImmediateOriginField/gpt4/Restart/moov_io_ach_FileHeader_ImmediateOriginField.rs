
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, String>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchFileHeader {
    immediate_origin: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&mut self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        self.immediate_origin = self.immediate_origin.trim().to_string();
        if let Some(ref opts) = self.validate_opts {
            if opts.bypass_origin_validation && self.immediate_origin.len() == 10 {
                return self.immediate_origin.clone();
            }
        }
        " ".to_owned() + &MoovIoAchConverters::string_field(&self.immediate_origin, 9)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as i32;
        let pad = unsafe {
            MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap()
                .clone()
        };
        pad + s
    }
}

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
