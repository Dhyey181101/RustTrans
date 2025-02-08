
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99 {
    pub original_trace: String,
    pub date_of_death: Option<String>,
    pub original_dfi: Option<String>,
    pub addenda_information: Option<String>,
    pub trace_number: Option<String>,
}

impl MoovIoAchAddenda99 {
    pub fn original_trace_field(&self) -> String {
        self.string_field(&self.original_trace, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

fn moov_io_ach_string_zeros(max: usize) -> String {
    let mut out = String::new();
    for _ in 0..max {
        out.push('0');
    }
    out
}

#[derive(Debug)]
pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = moov_io_ach_string_zeros(m);
        pad + s
    }
}

#[derive(Debug)]
pub struct MoovIoAchAddenda99Builder {
    original_trace: Option<String>,
    date_of_death: Option<String>,
    original_dfi: Option<String>,
    addenda_information: Option<String>,
    trace_number: Option<String>,
}

impl MoovIoAchAddenda99Builder {
    pub fn new() -> Self {
        Self {
            original_trace: None,
            date_of_death: None,
            original_dfi: None,
            addenda_information: None,
            trace_number: None,
        }
    }

    pub fn original_trace(mut self, original_trace: String) -> Self {
        self.original_trace = Some(original_trace);
        self
    }

    pub fn date_of_death(mut self, date_of_death: String) -> Self {
        self.date_of_death = Some(date_of_death);
        self
    }

    pub fn original_dfi(mut self, original_dfi: String) -> Self {
        self.original_dfi = Some(original_dfi);
        self
    }

    pub fn addenda_information(mut self, addenda_information: String) -> Self {
        self.addenda_information = Some(addenda_information);
        self
    }

    pub fn trace_number(mut self, trace_number: String) -> Self {
        self.trace_number = Some(trace_number);
        self
    }

    pub fn build(self) -> MoovIoAchAddenda99 {
        MoovIoAchAddenda99 {
            original_trace: self.original_trace.unwrap(),
            date_of_death: self.date_of_death,
            original_dfi: self.original_dfi,
            addenda_information: self.addenda_information,
            trace_number: self.trace_number,
        }
    }
}

impl FromStr for MoovIoAchAddenda99 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = MoovIoAchAddenda99Builder::new();
        for line in s.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0];
            let value = parts[1];
            match key {
                "originalTrace" => builder = builder.original_trace(value.to_string()),
                "dateOfDeath" => builder = builder.date_of_death(value.to_string()),
                "originalDFI" => builder = builder.original_dfi(value.to_string()),
                "addendaInformation" => builder = builder.addenda_information(value.to_string()),
                "traceNumber" => builder = builder.trace_number(value.to_string()),
                _ => {}
            }
        }
        Ok(builder.build())
    }
}

impl fmt::Display for MoovIoAchAddenda99 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "originalTrace={}", self.original_trace)?;
        if let Some(date_of_death) = &self.date_of_death {
            writeln!(f, "dateOfDeath={}", date_of_death)?;
        }
        if let Some(original_dfi) = &self.original_dfi {
            writeln!(f, "originalDFI={}", original_dfi)?;
        }
        if let Some(addenda_information) = &self.addenda_information {
            writeln!(f, "addendaInformation={}", addenda_information)?;
        }
        if let Some(trace_number) = &self.trace_number {
            writeln!(f, "traceNumber={}", trace_number)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moov_io_ach_addenda_99() {
        let addenda_99 = MoovIoAchAddenda99 {
            original_trace: "123456789012345".to_string(),
            date_of_death: None,
            original_dfi: None,
            addenda_information: None,
            trace_number: None,
        };

        assert_eq!(
            addenda_99.to_string(),
            "originalTrace=123456789012345"
        );

        let addenda_99 = MoovIoAchAddenda99 {
            original_trace: "123456789012345".to_string(),
            date_of_death: Some("20230101".to_string()),
            original_dfi: Some("123456789".to_string()),
            addenda_information: Some("This is some addenda information".to_string()),
            trace_number: Some("9876543210".to_string()),
        };

        assert_eq!(
            addenda_99.to_string(),
            "originalTrace=123456789012345\ndateOfDeath=20230101\noriginalDFI=123456789\naddendaInformation=This is some addenda information\ntraceNumber=9876543210"
        );

        let addenda_99 = MoovIoAchAddenda99::from_str(
            "originalTrace=123456789012345\ndateOfDeath=20230101\noriginalDFI=123456789\naddendaInformation=This is some addenda information\ntraceNumber=9876543210",
        )
        .unwrap();

        assert_eq!(
            addenda_99.to_string(),
            "originalTrace=123456789012345\ndateOfDeath=20230101\noriginalDFI=123456789\naddendaInformation=This is some addenda information\ntraceNumber=9876543210"
        );
    }
}
