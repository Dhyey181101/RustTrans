
use std::io::Read;

pub struct MoovIoAchReader<'a> {
    pub file: MoovIoAchFile,
    pub iat_current_batch: Vec<MoovIoAchBatch>,
    pub r: &'a mut dyn Read,
    pub line: String,
    pub current_batch: Vec<MoovIoAchBatch>,
    pub line_number: usize,
    pub max_lines: usize,
    pub record_name: String,
    pub errors: Vec<String>,
    pub skip_batch_accumulation: bool,
}

pub struct MoovIoAchValidateOpts {
    pub skip_all: bool,
    pub require_aba_origin: bool,
    pub bypass_origin_validation: bool,
    pub bypass_destination_validation: bool,
    pub check_transaction_code: bool,
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

pub struct MoovIoAchFile {
    pub header: MoovIoAchFileHeader,
    pub notification_of_change: Vec<MoovIoAchBatch>,
    pub return_entries: Vec<MoovIoAchBatch>,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchFileHeader {
    pub id: String,
    pub priority_code: String,
    pub immediate_destination: String,
    pub immediate_origin: String,
    pub file_creation_date: String,
    pub file_creation_time: String,
    pub file_id_modifier: String,
    pub record_size: String,
    pub blocking_factor: String,
    pub format_code: String,
    pub immediate_destination_name: String,
    pub immediate_origin_name: String,
    pub reference_code: String,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchBatch {
    pub id: String,
    pub service_class_code: String,
    pub entry_addenda_count: String,
    pub entry_hash: String,
    pub total_debit_amount: String,
    pub total_credit_amount: String,
    pub company_name: String,
    pub company_discretionary_data: String,
    pub originating_dfi_id: String,
    pub file_id: String,
    pub batch_number: String,
    pub validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

