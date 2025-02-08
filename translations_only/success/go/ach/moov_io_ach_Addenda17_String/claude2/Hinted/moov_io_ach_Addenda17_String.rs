

use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACE_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " "));
static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0"));

const MOOV_IO_ACH_RECORD_LENGTH: usize = 94;
const MOOV_IO_ACH_ENTRY_ADDENDA_POS: &str = "7";

struct MoovIoAchAddenda17 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda17 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(MOOV_IO_ACH_RECORD_LENGTH);
        buf.push_str(MOOV_IO_ACH_ENTRY_ADDENDA_POS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }
    
    fn payment_related_information_field(&self) -> String {
        self.converters.alpha_field(self.payment_related_information.clone(), 80)
    }
    
    fn sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let pad = self.get_pad(max - ln, &*MOOV_IO_ACH_SPACE_ZEROS);
            s + &pad
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let pad = self.get_pad(max - l, &*MOOV_IO_ACH_STRING_ZEROS);
            pad + &s
        }
    }

    fn get_pad(&self, n: u32, pad_map: &HashMap<usize, String>) -> String {
        if let Some(pad) = pad_map.get(&(n as usize)) {
            pad.clone()
        } else {
            " ".repeat(n as usize)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

