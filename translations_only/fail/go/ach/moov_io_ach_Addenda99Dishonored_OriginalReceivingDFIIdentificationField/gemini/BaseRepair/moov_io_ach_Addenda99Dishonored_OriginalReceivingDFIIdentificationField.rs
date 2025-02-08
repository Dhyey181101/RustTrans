
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Dishonored {
    pub original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn original_receiving_dfi_identification(&self) -> &str {
        &self.original_receiving_dfi_identification
    }
}

impl ToString for MoovIoAchAddenda99Dishonored {
    fn to_string(&self) -> String {
        format!(
            "OriginalReceivingDFIIdentification: {}",
            self.original_receiving_dfi_identification
        )
    }
}

impl FromStr for MoovIoAchAddenda99Dishonored {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let original_receiving_dfi_identification = parts[1].trim();
        Ok(MoovIoAchAddenda99Dishonored {
            original_receiving_dfi_identification: original_receiving_dfi_identification.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m as usize);
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out[&max].to_string()
}
