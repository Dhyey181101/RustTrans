
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

struct MoovIoAchAddenda11 {
    type_code: String,
    originator_name: String,
    originator_street_address: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda11 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_name_field());
        buf.push_str(&self.originator_street_address_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn originator_name_field(&self) -> String {
        alpha_field(&self.originator_name, 35)
    }

    fn originator_street_address_field(&self) -> String {
        alpha_field(&self.originator_street_address, 35)
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
        let pad = MOOV_IO_ACH_SPACE_ZEROS.get(&(max - len)).unwrap();
        format!("{}{}", s, pad)
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let len = s.len();

    if len > max {
        s[s.len() - max..].to_string()
    } else {
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(max - len)).unwrap();
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

