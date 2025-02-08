
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.effective_entry_date, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            match MOOV_IO_ACH_STRINGZEROS.get(&m) {
                Some(pad) => pad.to_owned() + s,
                None => "0".repeat(m as usize) + s,
            }
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

