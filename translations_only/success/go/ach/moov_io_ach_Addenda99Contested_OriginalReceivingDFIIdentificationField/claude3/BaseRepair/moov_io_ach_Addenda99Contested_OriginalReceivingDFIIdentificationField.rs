
use std::collections::HashMap;

struct MoovIoAchAddenda99Contested {
    original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Contested {
    fn original_receiving_dfi_identification_field(&self) -> String {
        self.string_field(&self.original_receiving_dfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        let m = (max - ln) as usize;
        let mut zeros = Vec::with_capacity(m);
        for _ in 0..m {
            zeros.push('0');
        }
        zeros.extend(s.chars());
        zeros.iter().collect()
    }
}
