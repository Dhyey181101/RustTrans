
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchFileHeader {
    file_creation_time: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_time_field(&self) -> String {
        match self.file_creation_time.chars().count() {
            0 => {
                let now = SystemTime::now();
                let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                let in_minutes = since_the_epoch.as_secs() / 60;
                format!("{:04}", in_minutes % 1440) // Assuming HHMM format for a day's minutes
            }
            4 => self.format_simple_time(&self.file_creation_time),
            _ => "".to_string(),
        }
    }

    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            MoovIoAchConverters::string_field(s, 4)
        } else {
            s.to_string()
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s.chars().take(max as usize).collect()
        } else {
            let m = (max - ln) as i32;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .as_ref()
                    .unwrap()
                    .get(&m)
                    .unwrap()
                    .clone()
            };
            format!("{}{}", pad, s)
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
