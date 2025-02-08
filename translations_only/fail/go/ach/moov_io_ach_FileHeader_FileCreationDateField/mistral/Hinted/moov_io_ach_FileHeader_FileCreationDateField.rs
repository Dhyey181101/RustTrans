
use regex::Regex;
use std::fmt;
use std::str;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, len: usize) -> String {
        let s = s.trim();
        format!("{: <width$}", s, width = len,)
    }

    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }

        let re = Regex::new(r"(\d\d\d\d)(\d\d)(\d\d)").unwrap();
        let caps = re.captures(s).unwrap();

        self.string_field(&caps[1], 4) + &self.string_field(&caps[2], 2) + &self.string_field(&caps[3], 2)
    }
}

fn main() {
    let moov_io_ach_converters = MoovIoAchConverters;
    let date_string = "20220301";
    println!("{}", moov_io_ach_converters.format_simple_date(date_string));

    let empty_string = "";
    println!("{}", moov_io_ach_converters.format_simple_date(empty_string));
}
