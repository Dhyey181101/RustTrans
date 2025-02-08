
use std::collections::HashMap;

const RECORD_LENGTH: usize = 94;
const ENTRY_ADDENDA_POS: &str = "7";

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

lazy_static::lazy_static! {
    static ref SPACE_ZEROS: HashMap<usize, String> = populate_map(94, " ");
    static ref STRING_ZEROS: HashMap<usize, String> = populate_map(94, "0");
}

#[derive(Default)]
struct Addenda16 {
    type_code: String,
    receiver_city_state_province: String,
    receiver_country_postal_code: String,
    entry_detail_sequence_number: i32,
}

impl Addenda16 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(RECORD_LENGTH);
        buf.push_str(ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_city_state_province_field());
        buf.push_str(&self.receiver_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_city_state_province_field(&self) -> String {
        self.alpha_field(&self.receiver_city_state_province, 35)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        if let Some(pad) = SPACE_ZEROS.get(&m) {
            return s.to_owned() + pad;
        }
        // slow path
        s.to_owned() + &" ".repeat(m)
    }

    fn receiver_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.receiver_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        if s.len() > max {
            return s.chars().rev().take(max).collect();
        }

        let m = max - s.len();
        if let Some(pad) = STRING_ZEROS.get(&m) {
            return pad.to_owned() + &s;
        }
        // slow path
        "0".repeat(m) + &s
    }
}
