
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
    trace_number: String,
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99Contested {
    fn trace_number_field(&self) -> String {
        self.converters.string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(ref zeros) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = zeros.get(&m) {
                    return format!("{}{}", pad, s);
                }
            }
        }

        // slow path
        "0".repeat(m) + s
    }
}

fn main() {
    // Example of how to initialize and use
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
    let converters = Box::new(MoovIoAchConverters {});
    let addenda99 = MoovIoAchAddenda99Contested {
        trace_number: "123456789012345".to_string(),
        converters,
    };
    println!("{}", addenda99.trace_number_field());
}
