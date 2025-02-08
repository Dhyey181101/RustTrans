

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
        let pad = MoovIoAchStringZeros::get_pad_string(&MoovIoAchStringZeros::new(max, '0'), m);
        pad + s
    }
}

struct MoovIoAchStringZeros {
    map: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize, zero: char) -> Self {
        let mut map = HashMap::new();
        for i in 0..=max {
            let value = (0..i).map(|_| zero).collect::<String>();
            map.insert(i, value);
        }
        MoovIoAchStringZeros { map }
    }

    fn get_pad_string(&self, n: usize) -> String {
        match self.map.get(&n) {
            Some(value) => value.clone(),
            None => "0".repeat(n),
        }
    }
}

struct MoovIoAchAddenda99Contested {
    return_reason_code: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Contested {
    fn return_reason_code(&self) -> String {
        self.string_field(&self.return_reason_code, 2)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ReturnReasonCode: {}",
            self.return_reason_code()
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99Contested {
        return_reason_code: "12345".to_string(),
        moov_io_ach_converters: MoovIoAchConverters,
    };

    println!("{}", addenda);
}

