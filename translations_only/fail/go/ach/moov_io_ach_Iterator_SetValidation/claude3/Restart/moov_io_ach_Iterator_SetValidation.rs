
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

pub struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

pub struct MoovIoAchReader {
    file: MoovIoAchFile,
    // IATCurrentBatch: IATBatch,
    // r: Reader,
    // line: String,
    // currentBatch: Batch,
    // line_number: u32,
    // maxLines: u32,
    // recordName: String,
    // errors: Vec<Error>,
    // skipBatchAccumulation: bool,
}

pub struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    // NotificationOfChange: Vec<BatchCOR>,
    // ReturnEntries: Vec<Batch>,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

pub struct MoovIoAchFileHeader {
    // ID: String,
    // PriorityCode: String,
    // ImmediateDestination: String,
    // ImmediateOrigin: String,
    // FileCreationDate: String,
    // FileCreationTime: String,
    // FileIDModifier: char,
    // RecordSize: u16,
    // BlockingFactor: u16,
    // FormatCode: u8,
    // ImmediateDestinationName: String,
    // ImmediateOriginName: String,
    // ReferenceCode: String,
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
        if opts.is_some() {
            self.file.set_validation(opts);
        }
    }
}

impl MoovIoAchFile {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opt) = opts {
            self.validate_opts = Some(opt.clone());
            self.header.set_validation(Some(opt));
        }
    }
}

impl MoovIoAchFileHeader {
    pub fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(opt) = opts {
            self.validate_opts = Some(opt);
        }
    }
}
