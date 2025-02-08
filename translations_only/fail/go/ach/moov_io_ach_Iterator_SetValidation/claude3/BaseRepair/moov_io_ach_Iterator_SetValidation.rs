

use std::boxed::Box;

pub struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

#[derive(Clone)]
pub struct MoovIoAchValidateOpts {
    // SkipAll: bool,
    // RequireABAOrigin: bool,
    // BypassOriginValidation: bool,
    // BypassDestinationValidation: bool,
    // CheckTransactionCode: fn() -> bool,
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

pub struct MoovIoAchReader {
    file: MoovIoAchFile,
    // IATCurrentBatch: IATBatch,
    // r: Reader,
    // line: String,
    // currentBatch: Batch,
    // line_number: u64,
    // max_lines: u64,
    // record_name: String,
    // errors: Vec<String>,
    // skip_batch_accumulation: bool,
}

pub struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    // NotificationOfChange: Vec<BatchCOR>,
    // ReturnEntries: Vec<Batch>,
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
    // validator: Validator,
    // converters: Converters,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchIterator {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(reader) = &mut self.reader {
            reader.set_validation(opts);
        }
    }
}

impl MoovIoAchReader {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opts) = opts {
            self.file.set_validation(Some(opts.clone()));
        }
    }
}

impl MoovIoAchFile {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opts) = opts {
            self.validate_opts = Some(opts.clone());
            self.header.set_validation(Some(opts));
        }
    }
}

impl MoovIoAchFileHeader {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opts) = opts {
            self.validate_opts = Some(opts);
        }
    }
}

