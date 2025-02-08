

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Dishonored {
    dishonored_return_reason_code: String,
    // ... other fields ...
    moov_io_ach_converters: Box<Converters>,
}

struct Converters {
    map: HashMap<usize, String>,
}

impl Addenda99Dishonored {
    fn dishonored_return_reason_code(&self) -> String {
        self.moov_io_ach_converters.string_field(self.dishonored_return_reason_code.clone(), 3)
    }
}

impl Converters {
    fn new() -> Converters {
        Converters {
            map: get_pad_string(10),
        }
    }

    fn string_field(&self, s: String, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            self.map[&m].clone() + &s
        }
    }
}

fn get_pad_string(n: usize) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, ZERO.repeat(i));
    }
    out
}

impl fmt::Display for Addenda99Dishonored {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... format and write Addenda99Dishonored fields to `f` ...
        Ok(())
    }
}

