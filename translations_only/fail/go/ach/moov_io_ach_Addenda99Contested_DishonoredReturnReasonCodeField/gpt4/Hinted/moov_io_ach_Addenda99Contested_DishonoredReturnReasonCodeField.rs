
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

struct MoovIoAchAddenda99Contested {
    dishonored_return_reason_code: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Contested {
    fn dishonored_return_reason_code_field(&self) -> String {
        self.moov_io_ach_converters.string_field(&self.dishonored_return_reason_code, 2)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
                format!("{}{}", pad, s)
            } else {
                // slow path
                "0".repeat(m) + s
            }
        }
    }
}

fn main() {
    // Example usage
    let converter = Box::new(MoovIoAchConverters {});
    let contested = MoovIoAchAddenda99Contested {
        dishonored_return_reason_code: "z\"*".to_string(),
        moov_io_ach_converters: converter,
    };

    println!("{}", contested.dishonored_return_reason_code_field());
}
