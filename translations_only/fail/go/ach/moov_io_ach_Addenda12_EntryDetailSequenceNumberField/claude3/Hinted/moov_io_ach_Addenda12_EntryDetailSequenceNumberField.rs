
use std::collections::HashMap;

static mut STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn entry_detail_sequence_number_field(addenda12: &Box<Addenda12>) -> String {
    numeric_field(addenda12.entry_detail_sequence_number, 7)
}

fn numeric_field(n: i64, max: u32) -> String {
    let mut s = n.to_string();
    if s.len() > max as usize {
        let l = s.len();
        s = s.split_off(l - max as usize);
    } else {
        let m = (max as usize) - s.len();
        let pad = unsafe {
            STRING_ZEROS
                .as_ref()
                .unwrap_or_else(|| {
                    STRING_ZEROS = Some(populate_map(94, "0".to_string()));
                    STRING_ZEROS.as_ref().unwrap()
                })
                .get(&m)
                .unwrap_or(&"".to_string())
                .clone()
        };
        s = pad + &s;
    }
    s
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct Addenda12 {
    entry_detail_sequence_number: i64,
}

struct Converters {}
