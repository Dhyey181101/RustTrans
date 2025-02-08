

use std::collections::HashMap;

struct MoovIoAchConverters;

struct MoovIoAchAddenda99 {
    date_of_death: String,
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAddenda99 {
    fn date_of_death_field(&self) -> String {
        if self.date_of_death.is_empty() {
            self.moov_io_ach_converters.alpha_field_box("", 6)
        } else {
            self.moov_io_ach_converters.format_simple_date_box(&self.date_of_death)
        }
    }
}

impl MoovIoAchConverters {
    fn alpha_field_box(&self, s: &str, max: usize) -> String {
        let ln = s.chars().filter(|c| c.is_whitespace()).count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_space_zeros(m);
            s.to_string() + &pad
        }
    }

    fn format_simple_date_box(&self, s: &str) -> String {
        if s.is_empty() {
            self.string_field_box("", 6)
        } else {
            s.to_string()
        }
    }

    fn string_field_box(&self, s: &str, max: usize) -> String {
        let ln = s.chars().filter(|c| !c.is_whitespace()).count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = get_string_zeros(m);
            pad + s
        }
    }
}

fn get_space_zeros(n: usize) -> String {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, " ".repeat(i));
    }
    map[&n].clone()
}

fn get_string_zeros(n: usize) -> String {
    let mut map = HashMap::new();
    for i in 0..94 {
        map.insert(i, "0".repeat(i));
    }
    map[&n].clone()
}

