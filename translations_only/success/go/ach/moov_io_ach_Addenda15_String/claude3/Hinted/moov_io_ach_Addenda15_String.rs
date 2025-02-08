
use std::collections::HashMap;
use once_cell::sync::Lazy;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

static moov_io_ach_space_zeros: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, " ".to_string().repeat(i));
    }
    out
});

static moov_io_ach_string_zeros: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".to_string().repeat(i));
    }
    out
});

struct MoovIoAchAddenda15 {
    type_code: String,
    receiver_id_number: String,
    receiver_street_address: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda15 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_id_number_field());
        buf.push_str(&self.receiver_street_address_field());
        buf.push_str("                                  ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_id_number_field(&self) -> String {
        self.alpha_field(&self.receiver_id_number, 15)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            if let Some(pad) = moov_io_ach_space_zeros.get(&m) {
                s.to_string() + pad
            } else {
                s.to_string() + &" ".repeat(m)
            }
        }
    }

    fn receiver_street_address_field(&self) -> String {
        self.alpha_field(&self.receiver_street_address, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            if let Some(pad) = moov_io_ach_string_zeros.get(&m) {
                pad.to_string() + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}
