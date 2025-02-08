
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchControl {
    batch_number: i32,
}

impl MoovIoAchBatchControl {
    fn batch_number_field(&self) -> String {
        self.numeric_field(self.batch_number, 7)
    }

    fn numeric_field(&self, n: i32, max: usize) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max {
            s[l - max..].to_string()
        } else {
            let m = max - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m) + &s
            }
        }
    }
}

fn main() {
    let bc = MoovIoAchBatchControl { batch_number: 1779042878 };
    println!("{}", bc.batch_number_field()); // Expected output: 9042878

    let bc = MoovIoAchBatchControl { batch_number: 67502080 };
    println!("{}", bc.batch_number_field()); // Expected output: 7502080

    let bc = MoovIoAchBatchControl { batch_number: 2778 };
    println!("{}", bc.batch_number_field()); // Expected output: 0002778

    let bc = MoovIoAchBatchControl { batch_number: -57084 };
    println!("{}", bc.batch_number_field()); // Expected output: 0-57084
}
