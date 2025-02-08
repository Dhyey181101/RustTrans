

use std::fmt;
use std::collections::HashMap;
use std::str;
use std::iter;
use once_cell::sync::Lazy;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len()-max as usize..].to_string()
        } else {
            let m = (max - s.len() as u32) as usize;
            match *MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap() {
                ref pad => pad.to_string() + &s,
            }
        }
    }
}

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, iter::repeat('0').take(i).collect::<String>());
    }
    out
});

struct MoovIoAchADVFileControl {
    block_count: i32,
    // other fields omitted for brevity
}

impl MoovIoAchADVFileControl {
    fn block_count_field(&self) -> String {
        MoovIoAchConverters.numeric_field(self.block_count, 6)
    }
    // other methods omitted for brevity
}

// other structs and implementations omitted for brevity

