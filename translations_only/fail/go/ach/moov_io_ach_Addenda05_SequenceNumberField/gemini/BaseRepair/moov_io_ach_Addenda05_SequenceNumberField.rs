
use std::collections::HashMap;

pub struct MoovIoAchAddenda05 {
    pub sequence_number: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAddenda05 {
    pub fn sequence_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.sequence_number, 4)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let m = max - s.len() as u32;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i as usize));
    }
    out
}
