

use std::str;
use std::string::String;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<Box<[String; 94]>> = Lazy::new(|| generate_padding_strings(94, " "));
static MOOV_IO_ACH_STRINGZEROS: Lazy<Box<[String; 94]>> = Lazy::new(|| generate_padding_strings(94, "0"));

const MOOV_IO_ACH_RECORDLENGTH: u32 = 94;
const MOOV_IO_ACH_ENTRYADDENDAPOS: &str = "7";

struct MoovIoAchAddenda16 {
    type_code: String,
    receiver_city_state_province: String,
    receiver_country_postal_code: String,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda16 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRYADDENDAPOS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_city_state_province_field());
        buf.push_str(&self.receiver_country_postal_code_field());
        buf.push_str("              ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }

    fn receiver_city_state_province_field(&self) -> String {
        alpha_field(&self.receiver_city_state_province, 35)
    }

    fn receiver_country_postal_code_field(&self) -> String {
        alpha_field(&self.receiver_country_postal_code, 35)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        numeric_field(self.entry_detail_sequence_number, 7)
    }
}

fn alpha_field(s: &str, max: usize) -> String {
    let ln = s.len();
    if ln > max {
        s[..max].to_string()
    } else {
        let padding = MOOV_IO_ACH_SPACEZEROS.get(max - ln as usize).unwrap().clone();
        s.to_string() + &padding
    }
}

fn numeric_field(n: i32, max: usize) -> String {
    let s = n.to_string();
    let l = s.len();
    if l > max {
        s[(l - max)..].to_string()
    } else {
        let padding = MOOV_IO_ACH_STRINGZEROS.get(max - l).unwrap().clone();
        padding + &s
    }
}

fn generate_padding_strings(max: usize, pad_char: &str) -> Box<[String; 94]> {
    let mut v = Vec::with_capacity(max);
    for i in 0..max {
        v.push(pad_char.repeat(i));
    }
    Box::try_from(v).unwrap()
}

