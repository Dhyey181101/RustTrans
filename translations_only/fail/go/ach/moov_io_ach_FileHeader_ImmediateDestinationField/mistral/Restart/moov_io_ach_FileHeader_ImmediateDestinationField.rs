

use std::collections::HashMap;
use std::fmt;
use std::str;
use once_cell::sync::Lazy;

const ZERO: &str = "0";

struct MoovIoAchFileHeader {
    immediate_destination: String,
    // ... other fields omitted for brevity
    moov_io_ach_converters: MoovIoAchConverters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            " ".repeat(10)
        } else {
            let trimmed = self.immediate_destination.trim();
            if self.validate_opts.as_ref().map_or(false, |opts| {
                opts.bypass_destination_validation && trimmed.len() == 10
            }) {
                trimmed.to_string()
            } else {
                format!(" {:<9}", trimmed)
            }
        }
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.len();
        if len > max {
            s[..max].to_string()
        } else {
            let padding_len = max - len;
            let padding = PADDING.get(&padding_len).unwrap_or(&ZERO).to_string();
            padding + s
        }
    }
}

fn populate_map(max: usize) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, ZERO.to_string());
    }
    out
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool,
    // ... other fields omitted for brevity
}

impl fmt::Display for MoovIoAchFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting code omitted for brevity
        write!(
            f,
            "ImmediateDestination: {}, ImmediateDestinationField: {}\n",
            self.immediate_destination, self.immediate_destination_field()
        )
    }
}

static PADDING: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 0..100 {
        let z = ZERO;
        map.insert(i, z);
    }
    map
});

