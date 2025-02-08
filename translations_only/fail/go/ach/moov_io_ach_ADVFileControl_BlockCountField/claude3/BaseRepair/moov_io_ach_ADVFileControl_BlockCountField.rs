
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(94);
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map
});

struct MoovIoAchAdvFileControl {
    block_count: usize,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvFileControl {
    fn block_count_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.block_count, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: usize, max: u32) -> String {
        let s = n.to_string();
        let l = s.len();
        if l > max as usize {
            s.chars().rev().take(max as usize).collect()
        } else {
            let m = (max as usize) - l;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"".to_string()).to_string();
            pad + &s
        }
    }
}
