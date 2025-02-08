
use std::collections::HashMap;
use std::iter;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct Addenda18 {
    sequence_number: i32,
}

impl Addenda18 {
    fn sequence_number_field(&self) -> String {
        numeric_field(self.sequence_number, 4)
    }
}

fn numeric_field(n: i32, max: u32) -> String {
    let s = n.to_string();
    if s.len() as u32 > max {
        s[s.len() - max as usize..].to_string()
    } else {
        let m = max - s.len() as u32;
        if let Some(pad) = MOOV_IO_ACH_STRINGZEROS.get(&(m as i32)) {
            format!("{}{}", pad, s)
        } else {
            iter::repeat("0").take(m as usize).collect::<String>() + &s
        }
    }
}

fn populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut map = HashMap::with_capacity(max as usize);
    for i in 0..max {
        map.insert(i, zero.clone()); 
    }
    map
}

struct Converters;

