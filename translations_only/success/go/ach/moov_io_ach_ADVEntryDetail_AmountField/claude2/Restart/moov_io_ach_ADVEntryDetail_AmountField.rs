
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchConverters;

struct MoovIoAchAdvEntryDetail {
    amount: i32,
}

impl MoovIoAchAdvEntryDetail {
    fn amount_field(&self) -> String {
        numeric_field(self.amount, 12)
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let l = s.len();
    
    if l > max {
        s[l-max..].to_string()
    } else {
        let m = max - l;
        match MOOV_IO_ACH_STRINGZEROS.get(&m) {
            Some(pad) => {
                format!("{}{}", pad, s)
            }
            None => {
                "0".repeat(m as usize) + &s
            }
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

