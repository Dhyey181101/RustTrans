
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchFileHeader {
    immediate_destination: String,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>, 
}

struct MoovIoAchValidateOpts {
    bypass_destination_validation: bool
}

impl MoovIoAchFileHeader {
    fn immediate_destination_field(&self) -> String {
        if self.immediate_destination.is_empty() {
            return " ".repeat(10);
        }
        let mut immediate_destination = self.immediate_destination.trim().to_string();
        if self.validate_opts.is_some()
            && self.validate_opts.as_ref().unwrap().bypass_destination_validation
            && immediate_destination.len() == 10  
        {
            return immediate_destination;
        }
        let mut dest = " ".to_string();
        dest.push_str(&MoovIoAchConverters::string_field(
            immediate_destination,
            9,
        ));
        dest
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&m).unwrap().to_string();
        pad + &s
    }
}

fn moov_io_ach_populate_map(max: i32, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as usize, zero.repeat(i as usize));
    }
    out
}

