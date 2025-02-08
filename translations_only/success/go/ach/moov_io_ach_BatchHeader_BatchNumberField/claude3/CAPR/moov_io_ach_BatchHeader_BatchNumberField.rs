

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchBatchHeader {
    batch_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {}

impl MoovIoAchBatchHeader {
    fn batch_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.batch_number, 7)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let mut s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s = s[l - max as usize..].to_string();
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).to_owned();
            s = pad + &s;
        }
        s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

