
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

static ISO8601_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_time: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFileHeader {
    fn file_creation_time_field(&self) -> String {
        match self.file_creation_time.chars().count() {
            0 => "0000".to_string(),
            4 => self.moov_io_ach_converters.format_simple_time(&self.file_creation_time),
            _ => "".to_string(),
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field(s, 4)
        } else {
            s.to_string()
        }
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m) + s
            }
        }
    }
}

fn main() {
    let converters = Box::new(MoovIoAchConverters {});
    let file_header = MoovIoAchFileHeader {
        file_creation_time: "2023-04-01T12:34:56+00:00".to_string(),
        moov_io_ach_converters: converters,
    };

    println!("File Creation Time Field: {}", file_header.file_creation_time_field());
}
