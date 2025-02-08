
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".to_string().repeat(i));
    }
    map
});

struct MoovIoAchAddenda99Contested {
    return_reason_code: String,
}

impl MoovIoAchAddenda99Contested {
    fn return_reason_code_field(&self) -> String {
        string_field(&self.return_reason_code, 2)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        match MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)) {
            Some(pad) => {
                let mut result = pad.clone();
                result.push_str(s);
                result
            }
            None => "0".to_string().repeat(m as usize) + s,
        }
    }
}

