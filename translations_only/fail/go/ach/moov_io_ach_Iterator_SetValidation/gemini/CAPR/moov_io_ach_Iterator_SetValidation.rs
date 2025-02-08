
use std::io::Read;

pub struct MoovIoAchIterator<'a> {
    reader: &'a mut MoovIoAchReader,
}

pub struct MoovIoAchValidateOpts {
    pub skip_all: bool,
    pub require_aba_origin: bool,
    pub bypass_origin_validation: bool,
    pub bypass_destination_validation: bool,
    pub check_transaction_code: Option<Box<dyn Fn(&str) -> bool>>,
    pub custom_trace_numbers: bool,
    pub allow_zero_batches: bool,
    pub allow_missing_file_header: bool,
    pub allow_missing_file_control: bool,
    pub bypass_company_identification_match: bool,
    pub custom_return_codes: bool,
    pub unequal_service_class_code: bool,
    pub allow_unordered_batch_numbers: bool,
    pub allow_invalid_check_digit: bool,
    pub unequal_addenda_counts: bool,
    pub preserve_spaces: bool,
    pub allow_invalid_amounts: bool,
}

pub struct MoovIoAchReader {
    pub file: MoovIoAchFile,
    pub iat_current_batch: Option<Box<MoovIoAchIatBatch>>,
    pub r: Box<dyn Read>,
    pub line: String,
    pub current_batch: Option<Box<MoovIoAchBatch>>,
    pub line_number: usize,
    pub max_lines: Option<usize>,
    pub record_name: Option<String>,
    pub errors: Vec<String>,
    pub skip_batch_accumulation: bool,
}

pub struct MoovIoAchFile {
    pub header: MoovIoAchFileHeader,
    pub notification_of_change: Vec<Box<MoovIoAchBatchCor>>,
    pub return_entries: Vec<Box<MoovIoAchBatch>>,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchFileHeader {
    pub id: Option<String>,
    pub priority_code: Option<String>,
    pub immediate_destination: Option<String>,
    pub immediate_origin: Option<String>,
    pub file_creation_date: Option<String>,
    pub file_creation_time: Option<String>,
    pub file_id_modifier: Option<String>,
    pub record_size: Option<String>,
    pub blocking_factor: Option<String>,
    pub format_code: Option<String>,
    pub immediate_destination_name: Option<String>,
    pub immediate_origin_name: Option<String>,
    pub reference_code: Option<String>,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchBatch {
    pub header: MoovIoAchBatchHeader,
    pub entries: Vec<Box<MoovIoAchEntry>>,
    pub addenda: Vec<Box<MoovIoAchAddenda>>,
    pub control: MoovIoAchBatchControl,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchBatchHeader {
    pub service_class_code: Option<String>,
    pub company_name: Option<String>,
    pub company_discretionary_data: Option<String>,
    pub company_identification: Option<String>,
    pub standard_entry_class_code: Option<String>,
    pub company_entry_description: Option<String>,
    pub effective_entry_date: Option<String>,
    pub originating_dfi_identification: Option<String>,
    pub batch_number: Option<String>,
}

pub struct MoovIoAchEntry {
    pub transaction_code: Option<String>,
    pub receiving_dfi_identification: Option<String>,
    pub check_digit: Option<String>,
    pub amount: Option<String>,
    pub individual_id_number: Option<String>,
    pub individual_name: Option<String>,
    pub discretionary_data: Option<String>,
    pub addenda_record_indicator: Option<String>,
    pub trace_number: Option<String>,
}

pub struct MoovIoAchAddenda {
    pub payment_related_information: Option<String>,
    pub addenda_type_code: Option<String>,
    pub addenda_sequence_number: Option<String>,
    pub entry_detail_sequence_number: Option<String>,
}

pub struct MoovIoAchBatchControl {
    pub service_class_code: Option<String>,
    pub entry_addenda_count: Option<String>,
    pub total_debit_amount: Option<String>,
    pub total_credit_amount: Option<String>,
}

pub struct MoovIoAchIatBatch {
    pub header: MoovIoAchIatBatchHeader,
    pub entries: Vec<Box<MoovIoAchIatEntry>>,
    pub control: MoovIoAchIatBatchControl,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchIatBatchHeader {
    pub service_class_code: Option<String>,
    pub company_name: Option<String>,
    pub company_discretionary_data: Option<String>,
    pub company_identification: Option<String>,
    pub standard_entry_class_code: Option<String>,
    pub company_entry_description: Option<String>,
    pub effective_entry_date: Option<String>,
    pub originating_dfi_identification: Option<String>,
    pub batch_number: Option<String>,
}

pub struct MoovIoAchIatEntry {
    pub transaction_code: Option<String>,
    pub receiving_dfi_identification: Option<String>,
    pub check_digit: Option<String>,
    pub amount: Option<String>,
    pub individual_id_number: Option<String>,
    pub individual_name: Option<String>,
    pub discretionary_data: Option<String>,
    pub addenda_record_indicator: Option<String>,
    pub trace_number: Option<String>,
}

pub struct MoovIoAchIatBatchControl {
    pub service_class_code: Option<String>,
    pub entry_addenda_count: Option<String>,
    pub total_debit_amount: Option<String>,
    pub total_credit_amount: Option<String>,
}

pub struct MoovIoAchBatchCor {
    pub service_class_code: Option<String>,
    pub company_name: Option<String>,
    pub company_discretionary_data: Option<String>,
    pub company_identification: Option<String>,
    pub standard_entry_class_code: Option<String>,
    pub company_entry_description: Option<String>,
    pub effective_entry_date: Option<String>,
    pub originating_dfi_identification: Option<String>,
    pub batch_number: Option<String>,
    pub addenda_record_indicator: Option<String>,
    pub change_code: Option<String>,
    pub corrected_entry_or_addenda_record: Option<String>,
    pub trace_number: Option<String>,
}
