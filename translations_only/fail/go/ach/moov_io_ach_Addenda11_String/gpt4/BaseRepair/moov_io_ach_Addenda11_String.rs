
use std::collections::HashMap;

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

static mut MOOV_IO_ACH_SPACE_ZEROS: Option<HashMap<usize, String>> = None;
static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda11 {
    type_code: String,
    originator_name: String,
    originator_street_address: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda11 {
    fn new(type_code: String, originator_name: String, originator_street_address: String, entry_detail_sequence_number: i32) -> Self {
        Self {
            type_code,
            originator_name,
            originator_street_address,
            entry_detail_sequence_number,
        }
    }

    fn string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.originator_name_field());
        buf.push_str(&self.originator_street_address_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn originator_name_field(&self) -> String {
        self.alpha_field(&self.originator_name, 35)
    }

    fn originator_street_address_field(&self) -> String {
        self.alpha_field(&self.originator_street_address, 35)
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
            let pad = unsafe { MOOV_IO_ACH_SPACE_ZEROS.as_ref().unwrap().get(&m).unwrap() };
            format!("{}{}", s, pad)
        }
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s.chars().skip(l - max).collect()
        } else {
            let m = max - l;
            let pad = unsafe { MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap() };
            format!("{}{}", pad, s)
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_SPACE_ZEROS = Some(moov_io_ach_populate_map(MOOV_IO_ACH_RECORD_LENGTH, " "));
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(MOOV_IO_ACH_RECORD_LENGTH, "0"));
    }
}
