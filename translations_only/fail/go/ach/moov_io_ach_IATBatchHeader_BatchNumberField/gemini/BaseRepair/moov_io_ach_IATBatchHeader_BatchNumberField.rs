
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchIatBatchHeader {
    pub batch_number: i32,
}

impl MoovIoAchIatBatchHeader {
    pub fn batch_number_field(&self) -> String {
        self.numeric_field(self.batch_number, 7)
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> =
        moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i as usize));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_number_field() {
        let iat_bh = MoovIoAchIatBatchHeader { batch_number: 123456 };
        assert_eq!(iat_bh.batch_number_field(), "00123456");
    }

    #[test]
    fn test_numeric_field() {
        let iat_bh = MoovIoAchIatBatchHeader { batch_number: 123456 };
        assert_eq!(iat_bh.numeric_field(123456, 7), "00123456");
        assert_eq!(iat_bh.numeric_field(12345, 7), "00012345");
        assert_eq!(iat_bh.numeric_field(1234, 7), "00001234");
        assert_eq!(iat_bh.numeric_field(123, 7), "00000123");
        assert_eq!(iat_bh.numeric_field(12, 7), "00000012");
        assert_eq!(iat_bh.numeric_field(1, 7), "00000001");
        assert_eq!(iat_bh.numeric_field(0, 7), "00000000");
    }
}
