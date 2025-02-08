
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchFileControl {
    total_credit: i32
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.total_credit, 12)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len()-max as usize..].to_string()
        } else {
            let m = max - s.len() as u32;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

