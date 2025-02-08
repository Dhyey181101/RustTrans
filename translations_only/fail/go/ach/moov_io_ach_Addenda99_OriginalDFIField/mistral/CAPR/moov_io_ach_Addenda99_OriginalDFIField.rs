

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MoovIoAchStringZeros::new().get_zeros(m);
        pad + s
    }
}

struct MoovIoAchStringZeros {
    zeros: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new() -> Self {
        let mut zeros = HashMap::new();
        for i in 0..100 {
            zeros.insert(i, "0".repeat(i));
        }
        MoovIoAchStringZeros { zeros }
    }

    fn get_zeros(&self, n: usize) -> String {
        match self.zeros.get(&n) {
            Some(z) => z.clone(),
            None => "0".repeat(n),
        }
    }
}

struct MoovIoAchAddenda99 {
    original_dfi: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99 {
    fn original_dfi_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.original_dfi, 8)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OriginalDFI: {}",
            self.original_dfi_field()
        )
    }
}

fn main() {
    let addenda99 = MoovIoAchAddenda99 {
        original_dfi: "12345678".to_string(),
        moov_io_ach_converters: MoovIoAchConverters,
    };
    println!("{}", addenda99);
}

