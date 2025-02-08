
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct BatchControl {
    batch_number: i32
}

impl BatchControl {
    fn batch_number_field(&self) -> String {
        numeric_field(self.batch_number, 7)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len()-max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        let pad = MOV_IO_ACH_STRING_ZEROS.get(&(m as i32)).unwrap();
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::with_capacity(max as usize);
    for i in 0..max {
        map.insert(i, zero.repeat(i as usize));
    }
    map
}

struct Converters;

