
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::new(zero.repeat(i)));
    }
    out
}

struct MoovIoAchBatchControl {
    batch_number: usize,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> Box<String> {
        self.numeric_field(self.batch_number, 7)
    }

    fn numeric_field(&self, n: usize, max: u32) -> Box<String> {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            let mut result = String::new();
            for c in s.chars().rev().take(max as usize) {
                result.push(c);
            }
            Box::new(result)
        } else {
            let m = (max as usize) - l;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .get_or_insert_with(|| moov_io_ach_populate_map(95, "0"))
                    .get(&m)
                    .cloned()
                    .unwrap_or_default()
            };
            Box::new(pad.to_string() + &s)
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {}
