
use std::collections::HashMap;
use std::fmt::Write;

pub struct Addenda17 {
    pub sequence_number: i32,
}

pub struct Converters {}

impl Addenda17 {
    pub fn sequence_number_field(&self) -> String {
        let binding = string_zeros();
        let pad = binding.get(&((4 as usize) - self.sequence_number.to_string().len())).unwrap();
        format!("{}{}", pad, self.sequence_number)
    }
}

impl Converters {}

fn string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
}
