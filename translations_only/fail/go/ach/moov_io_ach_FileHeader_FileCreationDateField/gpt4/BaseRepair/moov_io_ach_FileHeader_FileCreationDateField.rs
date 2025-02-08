
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
    file_creation_date: String,
}

impl MoovIoAchFileHeader {
    fn file_creation_date_field(&self) -> String {
        match self.file_creation_date.chars().count() {
            0 => current_date_formatted(),
            6 => self.file_creation_date.clone(),
            _ => "".to_string(),
        }
    }
}

fn current_date_formatted() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_seconds = since_the_epoch.as_secs();
    let days_since_epoch = in_seconds / 86400;
    format!("{:06}", days_since_epoch % 1000000)
}

fn main() {
    let header = MoovIoAchFileHeader {
        file_creation_date: "20230401".to_string(),
    };
    println!("File Creation Date Field: {}", header.file_creation_date_field());
}
