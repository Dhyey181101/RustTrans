
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda98Refused {
    original_dfi: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda98Refused {
    fn new(original_dfi: String, moov_io_ach_converters: Converters) -> Self {
        Self {
            original_dfi,
            moov_io_ach_converters,
        }
    }
}

struct EntryField {
    name: String,
    value: String,
}

impl EntryField {
    fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

struct Format9 {
    service_class_code: EntryField,
    company_name: EntryField,
    company_discretionary_data: EntryField,
    company_identification: EntryField,
    addenda_type: EntryField,
    addenda_record_information: EntryField,
    addenda_details: HashMap<String, String>,
}

impl Format9 {
    fn new(
        service_class_code: EntryField,
        company_name: EntryField,
        company_discretionary_data: EntryField,
        company_identification: EntryField,
        addenda_type: EntryField,
        addenda_record_information: EntryField,
        addenda_details: HashMap<String, String>,
    ) -> Self {
        Self {
            service_class_code,
            company_name,
            company_discretionary_data,
            company_identification,
            addenda_type,
            addenda_record_information,
            addenda_details,
        }
    }
}

impl fmt::Display for Format9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}|{}|{}|{}|{}|{}|",
            self.service_class_code.value,
            self.company_name.value,
            self.company_discretionary_data.value,
            self.company_identification.value,
            self.addenda_type.value,
            self.addenda_record_information.value
        )?;

        for (key, value) in &self.addenda_details {
            writeln!(f, "{}|{}", key, value)?;
        }

        Ok(())
    }
}
