
use lazy_static::lazy_static;

pub struct MoovIoAchAdvEntryDetail {
    pub amount: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvEntryDetail {
    pub fn amount_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.amount, 12)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = &moov_io_ach_string_zeros[m as usize];
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: [String; 94] = {
        let mut out = Vec::with_capacity(94);
        for i in 0..94 {
            out.push(String::from_utf8(vec![b'0'; i as usize]).unwrap());
        }
        out.try_into().unwrap()
    };
}
