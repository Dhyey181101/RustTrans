
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, " ".to_string()));
static MOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

const MOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

struct Addenda15 {
    type_code: String,    
    receiver_id_number: String,   
    receiver_street_address: String,
    entry_detail_sequence_number: i32,
    converters: Box<Converters>,
}

struct Converters;

impl Addenda15 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(MOV_IO_ACH_RECORD_LENGTH);
        buf.push_str(MOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_id_number_field());
        buf.push_str(&self.receiver_street_address_field());
        buf.push_str("                                  ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_id_number_field(&self) -> String {
        alpha_field(&self.receiver_id_number, 15)
    }

    fn receiver_street_address_field(&self) -> String {
        alpha_field(&self.receiver_street_address, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn alpha_field(s: &str, max: usize) -> String {
    let len = s.len();
    if len > max {
        s[..max].to_string()
    } else {
        let pad_len = max - len;
        let pad = MOV_IO_ACH_SPACE_ZEROS.get(&pad_len).cloned().unwrap_or_else(|| " ".repeat(pad_len));
        format!("{}{}", s, pad)
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let len = s.len();

    if len > max {
        s[s.len()-max..].to_string()
    } else {
        let pad_len = max - len;
        let pad = MOV_IO_ACH_STRING_ZEROS.get(&pad_len).cloned().unwrap_or_else(|| "0".repeat(pad_len));
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut map = HashMap::with_capacity(max);
    for i in 0..max {
        map.insert(i, zero.repeat(i));
    }
    map
}

