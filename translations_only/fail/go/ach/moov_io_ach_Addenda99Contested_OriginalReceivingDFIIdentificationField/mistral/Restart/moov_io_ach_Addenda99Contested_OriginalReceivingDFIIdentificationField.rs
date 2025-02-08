
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Contested {
    fn new(
        original_receiving_dfi_identification: String,
        moov_io_ach_converters: Converters,
    ) -> Self {
        Self {
            original_receiving_dfi_identification,
            moov_io_ach_converters,
        }
    }
}

struct EntryDetail {
    sequence_number: i32,
    transaction_amount: String,
    addenda_type: String,
    addenda_record_identifier: String,
    addenda_output: String,
}

impl EntryDetail {
    fn new(
        sequence_number: i32,
        transaction_amount: String,
        addenda_type: String,
        addenda_record_identifier: String,
        addenda_output: String,
    ) -> Self {
        Self {
            sequence_number,
            transaction_amount,
            addenda_type,
            addenda_record_identifier,
            addenda_output,
        }
    }
}

struct Control {
    company_identification: String,
    company_discretionary_data: String,
    company_entry_description: String,
    service_class_code: String,
    company_name: String,
    street_address: String,
    city_state: String,
    zip_code: String,
    entry_addenda_count: i32,
}

impl Control {
    fn new(
        company_identification: String,
        company_discretionary_data: String,
        company_entry_description: String,
        service_class_code: String,
        company_name: String,
        street_address: String,
        city_state: String,
        zip_code: String,
        entry_addenda_count: i32,
    ) -> Self {
        Self {
            company_identification,
            company_discretionary_data,
            company_entry_description,
            service_class_code,
            company_name,
            street_address,
            city_state,
            zip_code,
            entry_addenda_count,
        }
    }
}

struct BatchHeader {
    company_identification: String,
    company_discretionary_data: String,
    service_class_code: String,
    batch_number: i32,
    company_name: String,
    street_address: String,
    city_state: String,
    zip_code: String,
    entry_addenda_count: i32,
    total_debit_dollar_amount_in_batch: String,
    total_credit_dollar_amount_in_batch: String,
}

impl BatchHeader {
    fn new(
        company_identification: String,
        company_discretionary_data: String,
        service_class_code: String,
        batch_number: i32,
        company_name: String,
        street_address: String,
        city_state: String,
        zip_code: String,
        entry_addenda_count: i32,
        total_debit_dollar_amount_in_batch: String,
        total_credit_dollar_amount_in_batch: String,
    ) -> Self {
        Self {
            company_identification,
            company_discretionary_data,
            service_class_code,
            batch_number,
            company_name,
            street_address,
            city_state,
            zip_code,
            entry_addenda_count,
            total_debit_dollar_amount_in_batch,
            total_credit_dollar_amount_in_batch,
        }
    }
}

struct FileControl {
    company_identification: String,
    company_discretionary_data: String,
    company_name: String,
    street_address: String,
    city_state: String,
    zip_code: String,
    number_of_batches_in_file: i32,
    total_debit_dollar_amount_in_file: String,
    total_credit_dollar_amount_in_file: String,
    company_entry_description: String,
    reserved_for_future_use: String,
}

impl FileControl {
    fn new(
        company_identification: String,
        company_discretionary_data: String,
        company_name: String,
        street_address: String,
        city_state: String,
        zip_code: String,
        number_of_batches_in_file: i32,
        total_debit_dollar_amount_in_file: String,
        total_credit_dollar_amount_in_file: String,
        company_entry_description: String,
        reserved_for_future_use: String,
    ) -> Self {
        Self {
            company_identification,
            company_discretionary_data,
            company_name,
            street_address,
            city_state,
            zip_code,
            number_of_batches_in_file,
            total_debit_dollar_amount_in_file,
            total_credit_dollar_amount_in_file,
            company_entry_description,
            reserved_for_future_use,
        }
    }
}

impl fmt::Display for EntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "SEQNUM: {} TRANSACTION_AMOUNT: {} ADDENDA_TYPE: {} ADDENDA_RECORD_IDENTIFIER: {} ADDENDA_OUTPUT: {}",
            self.sequence_number, self.transaction_amount, self.addenda_type, self.addenda_record_identifier, self.addenda_output
        )
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "COMPANY_IDENTIFICATION: {} COMPANY_DISCRETIONARY_DATA: {} COMPANY_ENTRY_DESCRIPTION: {} SERVICE_CLASS_CODE: {} COMPANY_NAME: {} STREET_ADDRESS: {} CITY_STATE: {} ZIP_CODE: {} ENTRY_ADENDA_COUNT: {}",
            self.company_identification, self.company_discretionary_data, self.company_entry_description, self.service_class_code, self.company_name, self.street_address, self.city_state, self.zip_code, self.entry_addenda_count
        )
    }
}

impl fmt::Display for BatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "COMPANY_IDENTIFICATION: {} COMPANY_DISCRETIONARY_DATA: {} SERVICE_CLASS_CODE: {} BATCH_NUMBER: {} COMPANY_NAME: {} STREET_ADDRESS: {} CITY_STATE: {} ZIP_CODE: {} ENTRY_ADENDA_COUNT: {} TOTAL_DEBIT_DOLLAR_AMOUNT_IN_BATCH: {} TOTAL_CREDIT_DOLLAR_AMOUNT_IN_BATCH: {}",
            self.company_identification, self.company_discretionary_data, self.service_class_code, self.batch_number, self.company_name, self.street_address, self.city_state, self.zip_code, self.entry_addenda_count, self.total_debit_dollar_amount_in_batch, self.total_credit_dollar_amount_in_batch
        )
    }
}

impl fmt::Display for FileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "COMPANY_IDENTIFICATION: {} COMPANY_DISCRETIONARY_DATA: {} COMPANY_NAME: {} STREET_ADDRESS: {} CITY_STATE: {} ZIP_CODE: {} NUMBER_OF_BATCHES_IN_FILE: {} TOTAL_DEBIT_DOLLAR_AMOUNT_IN_FILE: {} TOTAL_CREDIT_DOLLAR_AMOUNT_IN_FILE: {} COMPANY_ENTRY_DESCRIPTION: {} RESERVED_FOR_FUTURE_USE: {}",
            self.company_identification, self.company_discretionary_data, self.company_name, self.street_address, self.city_state, self.zip_code, self.number_of_batches_in_file, self.total_debit_dollar_amount_in_file, self.total_credit_dollar_amount_in_file, self.company_entry_description, self.reserved_for_future_use
        )
    }
}
