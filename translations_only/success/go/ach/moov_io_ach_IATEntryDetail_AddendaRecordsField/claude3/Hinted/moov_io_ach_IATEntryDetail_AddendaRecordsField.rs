

use std::collections::HashMap;
use once_cell::sync::Lazy;

fn populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0"));

struct MoovIoAchIatEntryDetail {
    addenda_records: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchIatEntryDetail {
    fn addenda_records_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.addenda_records, 4)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            String::from(&s[(l - max) as usize..])
        } else {
            let m = (max - l) as usize;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap_or(&String::from("")).to_string();
            pad + &s
        }
    }
}

