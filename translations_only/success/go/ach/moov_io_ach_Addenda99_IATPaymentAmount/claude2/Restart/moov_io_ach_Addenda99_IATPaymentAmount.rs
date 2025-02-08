
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchAddenda99 {
    addenda_information: String,
}

impl MoovIoAchAddenda99 {
    fn iat_payment_amount(&mut self, s: String) {
        self.addenda_information = string_field(s, 10);
    }
}

struct MoovIoAchConverters;

fn string_field(s: String, max: u32) -> String {
    let ln = s.len() as u32;
    
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = (max - ln) as i32;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

