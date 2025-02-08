
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

const MOOV_IO_ACH_RECORDLENGTH: i32 = 94;
const MOOV_IO_ACH_ENTRYADDENDAPOS: &str = "7";

struct MoovIoAchAddenda05 {
    type_code: String,
    payment_related_information: String,
    sequence_number: i32,
    entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
    fn string(&self) -> String {
        let mut buf = String::with_capacity(94);
        buf.push_str(MOOV_IO_ACH_ENTRYADDENDAPOS);
        buf.push_str(&self.type_code);
        buf.push_str(&self.payment_related_information_field());
        buf.push_str(&self.sequence_number_field());
        buf.push_str(&self.entry_detail_sequence_number_field());
        buf
    }
    
    fn payment_related_information_field(&self) -> String {
        self.alpha_field(self.payment_related_information.to_string(), 80)
    }
    
    fn sequence_number_field(&self) -> String {
        self.numeric_field(self.sequence_number, 4)
    }

    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl MoovIoAchConverters for MoovIoAchAddenda05 {
    fn alpha_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as i32;
        let pad = MOOV_IO_ACH_SPACEZEROS.get(&m).cloned().unwrap_or_else(|| " ".repeat(m as usize));
        s + &pad
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l-max) as usize..].to_string();
        } else {
            let m = (max - l) as i32;
            let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m as usize));
            pad + &s
        }
    }
}

trait MoovIoAchConverters {
    fn alpha_field(&self, s: String, max: u32) -> String;
    fn numeric_field(&self, n: i32, max: u32) -> String;
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

