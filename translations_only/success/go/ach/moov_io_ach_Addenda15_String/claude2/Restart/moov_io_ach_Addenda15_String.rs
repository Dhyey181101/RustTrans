
use std::str;
use std::string::String;
use std::vec::Vec;

static MOOV_IO_ACH_SPACEZEROS: once_cell::sync::Lazy<[String; 94]> = once_cell::sync::Lazy::new(|| {
    generate_padding_strings(94, " ")
});

static MOOV_IO_ACH_STRINGZEROS: once_cell::sync::Lazy<[String; 94]> = once_cell::sync::Lazy::new(|| {
    generate_padding_strings(94, "0")  
});

const MOOV_IO_ACH_RECORDLENGTH: u32 = 94;
const MOOV_IO_ACH_ENTRYADDENDAPOS: &str = "7";

struct MoovIoAchAddenda15 {
    type_code: String,
    receiver_id_number: String,
    receiver_street_address: String,
    entry_detail_sequence_number: i32,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda15 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRYADDENDAPOS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.receiver_id_number_field());
        buf.push_str(&self.receiver_street_address_field());
        buf.push_str("                                  ");
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }
    
    fn receiver_id_number_field(&self) -> String {
        self.converters.alpha_field(&self.receiver_id_number, 15)
    }
    
    fn receiver_street_address_field(&self) -> String {
        self.converters.alpha_field(&self.receiver_street_address, 35)
    }
    
    fn entry_detail_sequence_number_field(&self) -> String {
        self.converters.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                pad.push(' ');
            }
            s.to_string() + &pad
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - l) {
                pad.push('0');
            }
            pad + &s
        }
    }
}

fn generate_padding_strings(max: usize, pad_char: &str) -> [String; 94] {
    let mut v = Vec::with_capacity(max);
    for i in 0..max {
        v.push(std::iter::repeat(pad_char).take(i).collect());
    }
    v.try_into().unwrap()
}

