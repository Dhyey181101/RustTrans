
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchIATBatchHeader {
    effective_entry_date: String,
}

impl MoovIoAchIATBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.effective_entry_date, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m) + s
            }
        }
    }
}

fn main() {
    let header = MoovIoAchIATBatchHeader {
        effective_entry_date: "**x*)x*".to_string(),
    };
    println!("{}", header.effective_entry_date_field());

    let header = MoovIoAchIATBatchHeader {
        effective_entry_date: "**+*[x*".to_string(),
    };
    println!("{}", header.effective_entry_date_field());

    let header = MoovIoAchIATBatchHeader {
        effective_entry_date: "".to_string(),
    };
    println!("{}", header.effective_entry_date_field());

    let header = MoovIoAchIATBatchHeader {
        effective_entry_date: "(".to_string(),
    };
    println!("{}", header.effective_entry_date_field());
}
