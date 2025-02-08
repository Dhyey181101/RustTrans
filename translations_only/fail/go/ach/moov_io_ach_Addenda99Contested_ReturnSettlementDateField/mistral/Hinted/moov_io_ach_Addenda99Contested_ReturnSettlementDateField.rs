

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Self {
        MoovIoAchConverters {
            moov_io_ach_string_zeros: Self::moov_io_ach_string_zeros(),
        }
    }

    fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = self
            .moov_io_ach_string_zeros
            .get(&m)
            .cloned()
            .unwrap_or(String::from(""));

        pad + s
    }
}

impl MoovIoAchAddenda99Contested {
    fn return_settlement_date_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.return_settlement_date, 3)
    }
}

struct MoovIoAchAddenda99Contested {
    return_settlement_date: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Contested {
    fn new() -> Self {
        MoovIoAchAddenda99Contested {
            return_settlement_date: String::new(),
            moov_io_ach_converters: Box::new(MoovIoAchConverters::new()),
        }
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ReturnSettlementDate: {}",
            self.return_settlement_date
        )
    }
}

