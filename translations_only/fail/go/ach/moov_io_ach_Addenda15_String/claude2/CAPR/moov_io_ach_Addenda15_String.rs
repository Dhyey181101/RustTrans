
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_SPACEZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, " ".to_string()));
static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

const MOOV_IO_ACH_RECORDLENGTH: i32 = 94;
const MOOV_IO_ACH_ENTRYADDENDAPOS: &str = "7";

struct MoovIoAchAddenda15 {
    type_code: String,
    receiver_id_number: String,
    receiver_street_address: String,
    entry_detail_sequence_number: i32,
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

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

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as i32;
            MOOV_IO_ACH_SPACEZEROS.get(&m).cloned().unwrap_or_else(|| " ".repeat(m as usize)) + s
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = (max - l) as i32;
            MOOV_IO_ACH_STRINGZEROS.get(&m).cloned().unwrap_or_else(|| "0".repeat(m as usize)) + &s
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<i32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

