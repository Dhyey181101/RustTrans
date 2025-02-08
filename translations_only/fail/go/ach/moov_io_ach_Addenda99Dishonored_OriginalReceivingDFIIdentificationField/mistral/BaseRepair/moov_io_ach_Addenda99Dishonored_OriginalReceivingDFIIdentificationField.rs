
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Dishonored {
    fn new(
        original_receiving_dfi_identification: String,
        moov_io_ach_converters: Converters,
    ) -> Addenda99Dishonored {
        Addenda99Dishonored {
            original_receiving_dfi_identification,
            moov_io_ach_converters,
        }
    }
}

struct Record9 {
    r#type: String,
    addenda_type: String,
    addenda_record_identifier: String,
    entry_addenda_information: HashMap<String, String>,
}

impl Record9 {
    fn new(
        r#type: String,
        addenda_type: String,
        addenda_record_identifier: String,
        entry_addenda_information: HashMap<String, String>,
    ) -> Record9 {
        Record9 {
            r#type,
            addenda_type,
            addenda_record_identifier,
            entry_addenda_information,
        }
    }
}

impl fmt::Display for Record9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:",
            self.r#type, self.addenda_type, self.addenda_record_identifier
        )?;

        for (key, value) in &self.entry_addenda_information {
            writeln!(f, "{}{}:{}", ZERO, key, value)?;
        }

        Ok(())
    }
}
