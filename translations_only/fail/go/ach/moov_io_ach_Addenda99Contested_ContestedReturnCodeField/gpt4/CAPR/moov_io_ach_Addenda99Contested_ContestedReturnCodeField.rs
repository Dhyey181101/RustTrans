
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda99Contested {
    contested_return_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn contested_return_code_field(&self) -> String {
        let ln = self.contested_return_code.chars().count();
        if ln > 3 {
            return self.contested_return_code.chars().take(3).collect();
        }

        let m = 3 - ln;
        unsafe {
            let pad = MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap_or(&String::new())
                .clone();
            pad + &self.contested_return_code
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
