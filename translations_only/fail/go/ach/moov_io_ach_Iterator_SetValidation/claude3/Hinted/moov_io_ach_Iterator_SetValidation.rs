
#[derive(Default)]
pub struct MoovIoAchValidateOpts {
    // SkipAll: bool,
    // RequireABAOrigin: bool,
    // BypassOriginValidation: bool,
    // BypassDestinationValidation: bool,
    // CheckTransactionCode: fn(&str) -> bool,
    // CustomTraceNumbers: bool,
    // AllowZeroBatches: bool,
    // AllowMissingFileHeader: bool,
    // AllowMissingFileControl: bool,
    // BypassCompanyIdentificationMatch: bool,
    // CustomReturnCodes: bool,
    // UnequalServiceClassCode: bool,
    // AllowUnorderedBatchNumebrs: bool,
    // AllowInvalidCheckDigit: bool,
    // UnequalAddendaCounts: bool,
    // PreserveSpaces: bool,
    // AllowInvalidAmounts: bool,
}

pub struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

pub struct MoovIoAchReader {
    file: MoovIoAchFile,
    // IATCurrentBatch: ...,
    // r: ...,
    // line: ...,
    // currentBatch: ...,
    // line_number: ...,
    // max_lines: ...,
    // record_name: ...,
    // errors: ...,
    // skip_batch_accumulation: ...,
}

pub struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    // NotificationOfChange: ...,
    // ReturnEntries: ...,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchFileHeader {
    // id: String,
    // priority_code: String,
    // immediate_destination: String,
    // immediate_origin: String,
    // file_creation_date: String,
    // file_creation_time: String,
    // file_id_modifier: String,
    // record_size: String,
    // blocking_factor: String,
    // format_code: String,
    // immediate_destination_name: String,
    // immediate_origin_name: String,
    // reference_code: String,
    // validator: ...,
    // converters: ...,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchIterator {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        if let Some(reader) = &mut self.reader {
            reader.set_validation(opts);
        }
    }
}

impl MoovIoAchReader {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.file.set_validation(opts);
    }
}

impl MoovIoAchFile {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.validate_opts = Some(opts);
        self.header.set_validation(Box::new(MoovIoAchValidateOpts::default()));
    }
}

impl MoovIoAchFileHeader {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.validate_opts = Some(opts);
    }
}
