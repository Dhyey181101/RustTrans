
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

struct MoovIoAchFileControl {
    batch_count: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchFileControl {
    fn batch_count_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.batch_count, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s.chars().skip((l - max) as usize).collect()
        } else {
            let m = (max - l) as usize;
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
    // Example usage
    let file_control = MoovIoAchFileControl {
        batch_count: 168460023,
        moov_io_ach_converters: Box::new(MoovIoAchConverters),
    };
    println!("{}", file_control.batch_count_field());
}
