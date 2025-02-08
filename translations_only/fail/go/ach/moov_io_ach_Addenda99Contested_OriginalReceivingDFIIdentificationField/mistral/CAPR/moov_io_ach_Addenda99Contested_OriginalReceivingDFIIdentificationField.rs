
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: Converters,
}

struct Converters {
    pad_strings: HashMap<usize, String>,
}

impl Addenda99Contested {
    fn new() -> Addenda99Contested {
        Addenda99Contested {
            original_receiving_dfi_identification: String::new(),
            moov_io_ach_converters: Converters {
                pad_strings: HashMap::new(),
            },
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested")
    }
}
