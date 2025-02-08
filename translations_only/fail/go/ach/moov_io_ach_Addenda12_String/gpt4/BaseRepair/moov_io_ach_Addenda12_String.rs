
use std::collections::HashMap;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

static mut MOOV_IO_ACH_SPACE_ZEROS: Option<HashMap<usize, String>> = None;
static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: char) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.to_string().repeat(i));
    }
    out
}

fn populate_globals() {
    unsafe {
        MOOV_IO_ACH_SPACE_ZEROS = Some(moov_io_ach_populate_map(94, ' '));
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, '0'));
    }
}

struct MoovIoAchAddenda12 {
    type_code: String,
    originator_city_state_province: String,
    originator_country_postal_code: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda12 {
    fn new(type_code: String, originator_city_state_province: String, originator_country_postal_code: String, entry_detail_sequence_number: i32) -> Box<Self> {
        Box::new(Self {
            type_code,
            originator_city_state_province,
            originator_country_postal_code,
            entry_detail_sequence_number,
        })
    }

    fn string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_city_state_province_field());
        buf.push_str(&self.originator_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn originator_city_state_province_field(&self) -> String {
        self.alpha_field(&self.originator_city_state_province, 35)
    }

    fn originator_country_postal_code_field(&self) -> String {
        self.alpha_field(&self.originator_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_SPACE_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        return s.to_string() + pad;
                    }
                }
            }
            s.to_string() + &" ".repeat(max - ln)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s.chars().skip(l - max).collect()
        } else {
            let m = max - l;
            unsafe {
                if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                    if let Some(pad) = map.get(&m) {
                        return pad.clone() + &s;
                    }
                }
            }
            "0".repeat(max - l) + &s
        }
    }
}

fn main() {
    populate_globals();
}
