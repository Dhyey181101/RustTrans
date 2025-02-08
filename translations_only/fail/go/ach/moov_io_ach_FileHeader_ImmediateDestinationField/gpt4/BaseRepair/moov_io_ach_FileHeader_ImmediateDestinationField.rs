
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<i32, Box<String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, Box<String>> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i as usize)));
    }
    out
}

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&mut self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        self.immediate_destination = self.immediate_destination.trim().to_string();
        if let Some(ref opts) = self.validate_opts {
            if opts.bypass_destination_validation && self.immediate_destination.len() == 10 {
                return self.immediate_destination.clone();
            }
        }
        " ".to_owned() + &MoovIoAchConverters::string_field(&self.immediate_destination, 9)
    }
}

struct MoovIoAchConverters;

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
                .map_or_else(|| Box::new("0".repeat(m as usize)), |b| b.clone())
        };
        (*pad).clone() + s
    }
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
