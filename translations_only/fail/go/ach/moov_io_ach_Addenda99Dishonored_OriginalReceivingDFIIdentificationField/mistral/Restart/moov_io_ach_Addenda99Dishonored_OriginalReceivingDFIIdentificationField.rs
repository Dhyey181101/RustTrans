
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

struct EntryDetailField {
    sequence_value: i32,
    data_element_number: i32,
    data_element_name: String,
    data_element_value: String,
}

impl EntryDetailField {
    fn new(
        sequence_value: i32,
        data_element_number: i32,
        data_element_name: String,
        data_element_value: String,
    ) -> EntryDetailField {
        EntryDetailField {
            sequence_value,
            data_element_number,
            data_element_name,
            data_element_value,
        }
    }
}

struct ServiceClassCodes {
    addenda_indicator: String,
    addenda_type: String,
    addenda_sequence_number: i32,
}

impl ServiceClassCodes {
    fn new(
        addenda_indicator: String,
        addenda_type: String,
        addenda_sequence_number: i32,
    ) -> ServiceClassCodes {
        ServiceClassCodes {
            addenda_indicator,
            addenda_type,
            addenda_sequence_number,
        }
    }
}

struct Company {
    company_discretionary_data_map: HashMap<String, String>,
}

impl Company {
    fn new() -> Company {
        Company {
            company_discretionary_data_map: HashMap::new(),
        }
    }

    fn set_company_discretionary_data(
        &mut self,
        key: String,
        value: String,
    ) {
        self.company_discretionary_data_map.insert(key, value);
    }

    fn get_company_discretionary_data(
        &self,
        key: &str,
    ) -> Option<&String> {
        self.company_discretionary_data_map.get(key)
    }
}

struct Addenda9 {
    addenda_record_indicator: String,
    service_class_codes: ServiceClassCodes,
    company_identification: String,
    company_descriptive_date: String,
    addenda_details: Vec<EntryDetailField>,
    company: Company,
}

impl Addenda9 {
    fn new(
        addenda_record_indicator: String,
        service_class_codes: ServiceClassCodes,
        company_identification: String,
        company_descriptive_date: String,
        addenda_details: Vec<EntryDetailField>,
        company: Company,
    ) -> Addenda9 {
        Addenda9 {
            addenda_record_indicator,
            service_class_codes,
            company_identification,
            company_descriptive_date,
            addenda_details,
            company,
        }
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "ADDENDA9: \n\
            Addenda Record Indicator: {}\n\
            Service Class Codes: \n\
                Addenda Indicator: {}\n\
                Addenda Type: {}\n\
                Addenda Sequence Number: {}\n\
            Company Identification: {}\n\
            Company Descriptive Date: {}\n\
            Addenda Details: \n",
            self.addenda_record_indicator,
            self.service_class_codes.addenda_indicator,
            self.service_class_codes.addenda_type,
            self.service_class_codes.addenda_sequence_number,
            self.company_identification,
            self.company_descriptive_date,
        )?;

        for detail in &self.addenda_details {
            writeln!(
                f,
                    "Sequence Value: {}\n\
                    Data Element Number: {}\n\
                    Data Element Name: {}\n\
                    Data Element Value: {}\n",
                    detail.sequence_value,
                    detail.data_element_number,
                    detail.data_element_name,
                    detail.data_element_value,
                )?;
        }

        Ok(())
    }
}
