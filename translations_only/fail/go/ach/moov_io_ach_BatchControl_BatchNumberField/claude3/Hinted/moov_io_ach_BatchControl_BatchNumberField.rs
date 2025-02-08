
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchControl {
    batch_number: u64,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> String {
        self.numeric_field(self.batch_number, 7)
    }

    fn numeric_field(&self, n: u64, max: u8) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            return s[(l - max as usize)..].to_string();
        } else {
            let m = (max as usize) - l;
            let pad = unsafe {
                if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                    MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(95, "0"));
                }
                MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap_or(&"".to_string()).clone()
            };
            pad + &s
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(95, "0"));
    }

    let batch_control = MoovIoAchBatchControl { batch_number: 3291286715092175404 };
    println!("{}", batch_control.batch_number_field());

    let batch_control = MoovIoAchBatchControl { batch_number: 743986933252379475 };
    println!("{}", batch_control.batch_number_field());

    let batch_control = MoovIoAchBatchControl { batch_number: 18446743332078167648 };
    println!("{}", batch_control.batch_number_field());

    let batch_control = MoovIoAchBatchControl { batch_number: 12587190073825341011 };
    println!("{}", batch_control.batch_number_field());
}
