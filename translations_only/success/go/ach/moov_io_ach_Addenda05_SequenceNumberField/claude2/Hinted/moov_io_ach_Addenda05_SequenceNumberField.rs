
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i as usize));
    }
    map
});

struct Addenda05 {
    sequence_number: i32,
}

impl Addenda05 {
    fn sequence_number_field(&self) -> String {
        Converters::numeric_field(self.sequence_number, 4)
    }
}

struct Converters;

impl Converters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[(s.len()-max as usize)..].to_string()
        } else {
            let m = (max - s.len() as u32) as i32;
            match MOOV_IO_ACH_STRINGZEROS.get(&m) {
                Some(pad) => format!("{}{}", pad, s),
                None => "0".repeat(m as usize) + &s,
            }
        }
    }
}
