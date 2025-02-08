

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
            return s[..max as usize].to_string();
        }

        let m = (max - len) as usize;
        let pad = MoovIoAchConverters::get_zeros(m);
        String::from_utf8_lossy(pad).to_string() + s
    }

    fn get_zeros(n: usize) -> &'static [u8] {
        if n < ZEROS.len() {
            &ZEROS[..n]
        } else {
            ZEROS
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i).to_string());
    }
    out
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AdviceRoutingNumber: {}/{:9}",
            self.advice_routing_number,
            self.advice_routing_number_field()
        )
    }
}

fn main() {
    let mut map = moov_io_ach_populate_map(100, "0");
    let mut ed = MoovIoAchAdvEntryDetail {
        advice_routing_number: "123456789".to_string(),
    };
    println!("{}", ed);
}

