

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Dishonored {
    dishonored_return_reason_code: String,
    // ... other fields ...
    converters: Converters,
}

impl Addenda99Dishonored {
    fn dishonored_return_reason_code(&self) -> String {
        self.string_field(self.dishonored_return_reason_code.clone(), 3)
    }
}

struct Converters;

impl Addenda99Dishonored {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_pad_string(m);
            format!("{}{}", pad, s)
        }
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, ZERO.repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for Addenda99Dishonored {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... implement formatting ...
        Ok(())
    }
}

