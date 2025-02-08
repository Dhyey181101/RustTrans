
#[derive(Clone)]
pub struct MoovIoAchValidateOpts {
    // ... (field definitions omitted for brevity)
}

pub struct MoovIoAchFileHeader {
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFileHeader {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.validate_opts = Some(opts);
    }
}

pub struct MoovIoAchFile {
    header: Box<MoovIoAchFileHeader>,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

impl MoovIoAchFile {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.validate_opts = Some(opts.clone());
        self.header.set_validation(opts);
    }
}

pub struct MoovIoAchReader {
    file: Box<MoovIoAchFile>,
}

impl MoovIoAchReader {
    pub fn set_validation(&mut self, opts: Box<MoovIoAchValidateOpts>) {
        self.file.set_validation(opts);
    }
}
