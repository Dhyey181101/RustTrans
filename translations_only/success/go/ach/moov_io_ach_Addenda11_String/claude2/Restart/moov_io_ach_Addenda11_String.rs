
use std::str;

static MOOV_IO_ACH_SPACEZEROS: [&str; 94] = [" "; 94];
static MOOV_IO_ACH_STRINGZEROS: [&str; 94] = ["0"; 94];

const MOOV_IO_ACH_RECORDLENGTH: u8 = 94;
const MOOV_IO_ACH_ENTRYADDENDAPOS: &str = "7";

struct Addenda11 {
    type_code: String,
    originator_name: String,
    originator_street_address: String,
    entry_detail_sequence_number: i32   
}

impl Addenda11 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRYADDENDAPOS);
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

fn alpha_field(s: &str, max: u8) -> String {
    let ln = s.len() as u8;
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

fn numeric_field(n: i32, max: u8) -> String {
    let s = n.to_string();
    let l = s.len() as u8;
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

