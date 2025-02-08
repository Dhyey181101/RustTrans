
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct BatchHeader {
    batch_number: i32,
}

impl BatchHeader {
    fn batch_number_field(&self) -> String {
        Converters::numeric_field(self.batch_number, 7)
    }
}

struct Converters;

impl Converters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[(s.len()-max as usize)..].to_string()
        } else {
            let mut result = "0".repeat(max as usize - s.len());
            result.push_str(&s);
            result
        }
    }
}

