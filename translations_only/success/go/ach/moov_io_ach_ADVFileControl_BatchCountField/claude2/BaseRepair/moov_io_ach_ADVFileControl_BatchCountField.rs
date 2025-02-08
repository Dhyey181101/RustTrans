
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    populate_map(94, "0".to_owned())
});

struct MoovIoAchAdvFileControl {
    batch_count: i32,
}

impl MoovIoAchAdvFileControl {
    fn batch_count_field(&self) -> String {
        numeric_field(self.batch_count, 6)
    }
}

struct MoovIoAchConverters;

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        let chars: Vec<char> = s.chars().rev().take(max as usize).collect();
        let rev_chars: Vec<char> = chars.into_iter().rev().collect();
        rev_chars.into_iter().collect()
    } else {
        let m = (max - s.len() as u32) as usize;
        match MOOV_IO_ACH_STRINGZEROS.get(&m) {
            Some(pad) => format!("{}{}", pad, s),
            None => "0".repeat(m) + &s,
        }
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

