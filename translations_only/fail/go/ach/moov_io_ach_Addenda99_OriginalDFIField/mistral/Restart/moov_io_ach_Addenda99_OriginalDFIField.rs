

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
        let pad = self.get_pad_string(m);
        pad + s
    }

    fn get_pad_string(&self, n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
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
            "OriginalDFI: {}, OriginalDFIField: {}",
            self.original_dfi, self.original_dfi_field()
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99 {
        original_dfi: "123456789".to_string(),
        moov_io_ach_converters: MoovIoAchConverters {},
    };
    println!("{}", addenda);
}

