
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchFileControl {
    block_count: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn block_count_field(&self) -> String {
        numeric_field(self.block_count, 6)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    let l = s.len() as u32;
    
    if l > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - l;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

