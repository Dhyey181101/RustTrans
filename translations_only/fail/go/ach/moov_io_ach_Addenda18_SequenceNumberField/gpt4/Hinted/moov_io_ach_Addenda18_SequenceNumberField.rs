
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

struct MoovIoAchAddenda18 {
    sequence_number: i32,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda18 {
    fn sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.sequence_number, 4)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() > max as usize {
            s[s.len() - max as usize..].to_string()
        } else {
            let m = max as usize - s.len();
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
    let converter = MoovIoAchConverters;
    let addenda18_example = MoovIoAchAddenda18 {
        sequence_number: 409141247,
        moov_io_ach_converters: Box::new(converter),
    };
    println!("{}", addenda18_example.sequence_number_field());
}
