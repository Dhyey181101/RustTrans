
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<i32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchIATBatchHeader {
    foreign_exchange_reference_indicator: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchIATBatchHeader {
    fn foreign_exchange_reference_indicator_field(&self) -> String {
        self.numeric_field(self.foreign_exchange_reference_indicator, 1)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as i32;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m as usize) + &s
            }
        }
    }
}

fn main() {
    // Example usage
    let iat_bh = MoovIoAchIATBatchHeader {
        foreign_exchange_reference_indicator: 1482194442,
    };
    println!("{}", iat_bh.foreign_exchange_reference_indicator_field());
}
