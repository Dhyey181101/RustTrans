
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
}

impl MoovIoAchFileControl {
    fn entry_addenda_count_field(&self) -> String {
        self.numeric_field(self.entry_addenda_count, 8)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = max - l;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m as usize) + &s
            }
        }
    }
}

fn main() {
    // Example usage
    let control = MoovIoAchFileControl {
        entry_addenda_count: 2114191435,
    };
    println!("{}", control.entry_addenda_count_field());
}
