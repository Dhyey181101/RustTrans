
use once_cell::sync::Lazy;

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
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

