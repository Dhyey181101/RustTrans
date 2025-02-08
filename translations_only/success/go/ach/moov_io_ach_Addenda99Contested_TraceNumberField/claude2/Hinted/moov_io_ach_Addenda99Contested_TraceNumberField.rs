
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

struct MoovIoAchAddenda99Contested {
    trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn trace_number_field(&self) -> String {
        string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        match MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)) {
            Some(pad) => {
                let mut result = pad.clone();
                result.push_str(s);
                result
            }
            None => "0".repeat(m as usize) + s,
        }
    }
}

