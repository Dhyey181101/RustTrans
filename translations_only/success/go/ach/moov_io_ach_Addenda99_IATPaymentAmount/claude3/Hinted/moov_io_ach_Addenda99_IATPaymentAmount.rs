
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| populate_map(94, Box::from("0")));

struct Addenda99 {
    addenda_information: String,
}

impl Addenda99 {
    fn iat_payment_amount(&mut self, s: String) {
        self.addenda_information = string_field(s, 10);
    }
}

fn string_field(s: String, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s[..max as usize].to_string();
    }

    let m = (max - ln) as usize;
    let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m);
    if let Some(pad) = pad {
        return pad.to_string() + &s;
    }

    "0".repeat(m) + &s
}

fn populate_map(max: usize, zero: Box<str>) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.to_string().repeat(i)));
    }
    out
}
