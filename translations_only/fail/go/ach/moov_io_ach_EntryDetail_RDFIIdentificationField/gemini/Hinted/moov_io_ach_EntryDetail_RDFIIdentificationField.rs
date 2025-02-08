
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct MoovIoAchEntryDetail {
    r_d_f_i_identification: String,
}

impl MoovIoAchEntryDetail {
    fn r_d_f_i_identification_field(&self) -> String {
        self.string_field(self.r_d_f_i_identification.clone(), 8)
    }

    fn string_field(&self, s: String, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + &s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].clone()
}

fn main() {
    let ed = MoovIoAchEntryDetail {
        r_d_f_i_identification: "".to_string(),
    };
    println!("{}", ed.r_d_f_i_identification_field());

    let ed = MoovIoAchEntryDetail {
        r_d_f_i_identification: "0".to_string(),
    };
    println!("{}", ed.r_d_f_i_identification_field());

    let ed = MoovIoAchEntryDetail {
        r_d_f_i_identification: "<\\\\\\".to_string(),
    };
    println!("{}", ed.r_d_f_i_identification_field());

    let ed = MoovIoAchEntryDetail {
        r_d_f_i_identification: "\\A@\\".to_string(),
    };
    println!("{}", ed.r_d_f_i_identification_field());
}
