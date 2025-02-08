

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    string_field_map: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        let mut map = HashMap::new();
        for i in 0..94 {
            map.insert(i, ZEROS.repeat(i));
        }
        MoovIoAchConverters {
            string_field_map: map,
        }
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            if let Some(pad) = self.string_field_map.get(&m) {
                format!("{}{}", pad, s)
            } else {
                format!("{}^{}", "0".repeat(m), s)
            }
        }
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.moov_io_ach_converters.string_field(self.rdfi_identification.clone(), 8))
    }
}

fn main() {
    let adv_entry_detail = MoovIoAchAdvEntryDetail {
        rdfi_identification: "/&".to_string(),
        moov_io_ach_converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", adv_entry_detail);
}

