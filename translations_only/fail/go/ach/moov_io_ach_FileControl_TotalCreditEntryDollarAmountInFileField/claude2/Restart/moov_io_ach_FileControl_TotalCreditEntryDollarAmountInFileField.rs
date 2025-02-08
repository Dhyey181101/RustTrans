
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(init);

fn init() -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i as usize));
    }
    map
}

struct MoovIoAchFileControl {
    total_credit: i32
}

impl MoovIoAchFileControl {
    fn total_credit_entry_dollar_amount_in_file_field(&self) -> String {
        numeric_field(self.total_credit, 12)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() as u32 > max {
        s.drain(..s.len() - max as usize);
        s
    } else {
        let m = max - s.len() as u32;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&(m as i32)) {
            format!("{}{}", pad, s)
        } else {
            s.drain(..0);
            s.push_str(&"0".repeat(m as usize));
            s
        }
    }
}

