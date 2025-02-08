

use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn alpha_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_space_zeros(m);
        s.to_string() + &pad
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.to_string()
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_string_zeros(m);
        pad + s
    }
}

struct MoovIoAchAddenda99 {
    date_of_death: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99 {
    fn date_of_death_field(&self) -> String {
        if self.date_of_death.is_empty() {
            return self.moov_io_ach_converters.alpha_field("", 6);
        }
        self.moov_io_ach_converters.format_simple_date(&self.date_of_death)
    }
}

fn get_space_zeros(n: usize) -> String {
    let mut space_zeros = HashMap::new();
    space_zeros.insert(n, " ".repeat(n) + &"0".repeat(n));

    String::from(space_zeros.get(&n).unwrap_or(&" ".repeat(n)))
}

fn get_string_zeros(n: usize) -> String {
    let mut string_zeros = HashMap::new();
    string_zeros.insert(n, "0".repeat(n));

    String::from(string_zeros.get(&n).unwrap_or(&"0".repeat(n)))
}

fn main() {
    let addenda99 = MoovIoAchAddenda99 {
        date_of_death: String::from(""),
        moov_io_ach_converters: MoovIoAchConverters,
    };

    println!("{}", addenda99.date_of_death_field());
}

