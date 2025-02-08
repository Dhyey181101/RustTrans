
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoovIoAchAddenda99 {
    pub date_of_death: Option<String>,
}

impl MoovIoAchAddenda99 {
    pub fn date_of_death_field(&self) -> String {
        match &self.date_of_death {
            Some(date) => date.clone(),
            None => String::from(" ").repeat(6),
        }
    }
}

impl FromStr for MoovIoAchAddenda99 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addenda99 = MoovIoAchAddenda99 {
            date_of_death: None,
        };

        for line in s.lines() {
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            match key {
                "DateOfDeath" => addenda99.date_of_death = Some(value.to_string()),
                _ => (),
            }
        }

        Ok(addenda99)
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DateOfDeath: {}", self.date_of_death_field())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn alpha_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = " ".repeat(m);
        s.to_string() + &pad
    }

    pub fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.to_string()
    }

    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = "0".repeat(m);
        pad + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_of_death_field() {
        let addenda99 = MoovIoAchAddenda99 {
            date_of_death: Some("20230101".to_string()),
        };
        assert_eq!(addenda99.date_of_death_field(), "20230101");

        let addenda99 = MoovIoAchAddenda99 {
            date_of_death: None,
        };
        assert_eq!(addenda99.date_of_death_field(), " ".repeat(6));
    }

    #[test]
    fn test_alpha_field() {
        let converters = MoovIoAchConverters {};
        assert_eq!(converters.alpha_field("abc", 3), "abc");
        assert_eq!(converters.alpha_field("abc", 5), "abc  ");
        assert_eq!(converters.alpha_field("abcdef", 5), "abcde");
    }

    #[test]
    fn test_format_simple_date() {
        let converters = MoovIoAchConverters {};
        assert_eq!(converters.format_simple_date("20230101"), "20230101");
        assert_eq!(converters.format_simple_date(""), "000000");
    }

    #[test]
    fn test_string_field() {
        let converters = MoovIoAchConverters {};
        assert_eq!(converters.string_field("123", 3), "123");
        assert_eq!(converters.string_field("123", 5), "00123");
        assert_eq!(converters.string_field("123456", 5), "12345");
    }

    #[test]
    fn test_moov_io_ach_populate_map() {
        let map = moov_io_ach_populate_map(10, "0");
        assert_eq!(map.get(&0), Some(&""));
        assert_eq!(map.get(&1), Some(&"0"));
        assert_eq!(map.get(&9), Some(&"000000000"));
    }

    #[test]
    fn test_from_str() {
        let addenda99 = "DateOfDeath: 20230101".parse::<MoovIoAchAddenda99>().unwrap();
        assert_eq!(addenda99.date_of_death, Some("20230101".to_string()));
    }

    #[test]
    fn test_display() {
        let addenda99 = MoovIoAchAddenda99 {
            date_of_death: Some("20230101".to_string()),
        };
        assert_eq!(
            addenda99.to_string(),
            "DateOfDeath: 20230101"
        );
    }
}
