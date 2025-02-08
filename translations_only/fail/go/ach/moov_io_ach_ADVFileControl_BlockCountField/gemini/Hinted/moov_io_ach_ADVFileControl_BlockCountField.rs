
use std::collections::HashMap;
use std::fmt::Write;

struct MoovIoAchAdvFileControl {
    block_count: i32,
}

struct MoovIoAchConverters {
    moov_io_ach_string_zeros: HashMap<i32, String>,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self, c: &MoovIoAchConverters) -> String {
        c.numeric_field(self.block_count, 6)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let mut pad = String::new();
            for _ in 0..m {
                pad.push('0');
            }
            return pad + &s;
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push_str(zero);
        }
        out.insert(i, s);
    }
    out
}

fn main() {
    let moov_io_ach_string_zeros = moov_io_ach_populate_map(94, "0");
    let fc = MoovIoAchAdvFileControl { block_count: 123 };
    let c = MoovIoAchConverters {
        moov_io_ach_string_zeros,
    };
    println!("{}", fc.block_count_field(&c));
}
