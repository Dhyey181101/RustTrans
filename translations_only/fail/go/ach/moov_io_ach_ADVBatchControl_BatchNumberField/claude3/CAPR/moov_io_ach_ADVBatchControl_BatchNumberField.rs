
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_numeric_field(n: usize, max: usize) -> String {
    let s = n.to_string();
    if s.len() > max {
        String::from(&s[s.len() - max..])
    } else {
        let m = max - s.len();
        let pad = unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap_or(&"".to_string()).clone()
        };
        pad + &s
    }
}

struct MoovIoAchADVBatchControl {
    batch_number: usize,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchADVBatchControl {
    fn batch_number_field(&self) -> String {
        moov_io_ach_numeric_field(self.batch_number, 7)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: usize) -> String {
        moov_io_ach_numeric_field(n, max)
    }
}
