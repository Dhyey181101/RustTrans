
use std::collections::HashMap;

struct MoovIoAchStringZeros(HashMap<i32, String>);

impl MoovIoAchStringZeros {
    fn new() -> Self {
        MoovIoAchStringZeros(populate_map())
    }
}

fn populate_map() -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}

struct MoovIoAchAddenda15 {
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda15 {
    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = (max - s.len() as u32) as i32;
        let pad = MoovIoAchStringZeros::new().0.get(&m).unwrap().clone();
        format!("{}{}", pad, s)
    }
}

