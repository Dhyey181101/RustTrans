
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FileHeader {
    pub file_creation_time: String,
}

impl FileHeader {
    pub fn file_creation_time_field(&self) -> String {
        match self.file_creation_time.len() {
            0 => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            4 => self.format_simple_time(&self.file_creation_time),
            _ => SystemTime::from(UNIX_EPOCH + Duration::from_secs(self.file_creation_time.parse::<u64>().unwrap()))
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        }
    }

    fn format_simple_time(&self, s: &str) -> String {
        if s.is_empty() {
            String::from("0000")
        } else {
            s.to_string()
        }
    }
}

pub fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_creation_time_field() {
        let fh = FileHeader {
            file_creation_time: "1504".to_string(),
        };
        assert_eq!(fh.file_creation_time_field(), "1504");

        let fh = FileHeader {
            file_creation_time: "20220308".to_string(),
        };
        assert_eq!(fh.file_creation_time_field(), "1646726400");
    }

    #[test]
    fn test_populate_map() {
        let out = populate_map(10, "0");
        assert_eq!(out.len(), 10);
        assert_eq!(out[&0], "");
        assert_eq!(out[&1], "0");
        assert_eq!(out[&9], "000000000");
    }
}
