

use std::collections::HashMap;
use std::str;

struct MoovIoAchFileHeader {
    immediate_origin: String,
    converters: MoovIoAchConverters,
    validate_opts: Option<MoovIoAchValidateOpts>,
}

struct MoovIoAchConverters;

struct MoovIoAchValidateOpts {
    bypass_origin_validation: bool,
}

impl MoovIoAchFileHeader {
    fn immediate_origin_field(&mut self) -> String {
        if self.immediate_origin.is_empty() {
            return " ".repeat(10);
        }
        self.immediate_origin = self.immediate_origin.trim().to_string();
        if let Some(ref validate_opts) = self.validate_opts {
            if validate_opts.bypass_origin_validation && self.immediate_origin.len() == 10 {
                return self.string_field(self.immediate_origin.clone(), 9);
            }
        }
        return " ".to_string() + &self.string_field(self.immediate_origin.clone(), 9);
    }

    fn string_field(&self, s: String, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = self.get_zeros(m);
        return pad + &s;
    }

    fn get_zeros(&self, n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        return out[&n].clone();
    }

    fn new() -> MoovIoAchFileHeader {
        MoovIoAchFileHeader {
            immediate_origin: String::new(),
            converters: MoovIoAchConverters {},
            validate_opts: None,
        }
    }
}

impl MoovIoAchValidateOpts {
    fn new() -> MoovIoAchValidateOpts {
        MoovIoAchValidateOpts {
            bypass_origin_validation: false,
        }
    }
}

