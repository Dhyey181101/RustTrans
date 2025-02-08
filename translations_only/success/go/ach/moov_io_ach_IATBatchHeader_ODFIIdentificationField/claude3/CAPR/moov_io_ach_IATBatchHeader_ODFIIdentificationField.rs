
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, Box<str>>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, Box::from("0".repeat(i)));
    }
    map
});

struct MoovIoAchIatBatchHeader {
    odfi_identification: String,
}

impl MoovIoAchIatBatchHeader {
    fn odfi_identification_field(&self) -> String {
        let mut result = self.odfi_identification.clone();
        if result.len() > 8 {
            result.truncate(8);
        } else {
            result = MOOV_IO_ACH_STRING_ZEROS.get(&(8 - result.len())).unwrap().to_string() + &result;
        }
        result
    }
}

struct MoovIoAchConverters;
