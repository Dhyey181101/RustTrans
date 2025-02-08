
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<i32, String>>> = None;

struct MoovIoAchEntryDetail {
    identification_number: String,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: String) {
        self.identification_number = format!(
            "{}{}",
            self.identification_number,
            MoovIoAchConverters::string_field(s, 11)
        );
    }
}

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
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
                .unwrap_or(&String::new())
                .clone()
        };
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> Box<HashMap<i32, String>> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    Box::new(out)
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
