

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchAdvEntryDetail {
    advice_routing_number: String,
}

impl MoovIoAchAdvEntryDetail {
    fn advice_routing_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.advice_routing_number, 9)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let len = s.len() as u32;
        if len > max {
            return s[..(max as usize)].to_string();
        }

        let m = (max - len) as usize;
        let pad = MoovIoAchConverters::get_zeros(m);
        String::from_utf8(pad.iter().cloned().chain(s.bytes()).collect()).unwrap()
    }

    fn get_zeros(n: usize) -> Box<[u8]> {
        if n < ZEROS.len() {
            ZEROS[..n].into()
        } else {
            ZEROS.into()
        }
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "adviceRoutingNumber:{}\n",
            self.advice_routing_number_field()
        )
    }
}

fn main() {
    let mut adv_entry = MoovIoAchAdvEntryDetail {
        advice_routing_number: "123456789".to_string(),
    };
    println!("{}", adv_entry);
}

