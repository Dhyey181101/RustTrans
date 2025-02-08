
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda17 {
    entry_detail_sequence_number: usize,
}

impl MoovIoAchAddenda17 {
    fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }

    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            String::from(&s[(s.len() - max as usize)..])
        } else {
            let m = max as usize - s.len();
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS
                    .get_or_insert_with(|| moov_io_ach_populate_map(95, "0"))
                    .get(&m)
                    .cloned()
            };
            if let Some(pad) = pad {
                pad + &s
            } else {
                "0".repeat(m) + &s
            }
        }
    }
}
