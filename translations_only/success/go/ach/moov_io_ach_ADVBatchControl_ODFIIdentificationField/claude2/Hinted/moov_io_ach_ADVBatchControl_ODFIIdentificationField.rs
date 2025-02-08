
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut out = HashMap::with_capacity(94);
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
});

struct MoovIoAchAdvBatchControl {
    odfi_identification: String,
}

impl MoovIoAchAdvBatchControl {
    fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            match MOOV_IO_ACH_STRINGZEROS.get(&m) {
                Some(pad) => pad.to_owned() + s,
                None => "0".repeat(m) + s,
            }
        }
    }
}

