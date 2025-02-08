
#[derive(Clone)]
struct MoovIoAchValidateOpts {
    // Fields as per Go struct
}

struct MoovIoAchIterator {
    reader: Option<Box<MoovIoAchReader>>,
}

struct MoovIoAchReader {
    file: MoovIoAchFile,
    // Other fields as per Go struct
}

struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchFileHeader {
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
    // Other fields as per Go struct
}

impl MoovIoAchIterator {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if let Some(ref mut reader) = self.reader {
            reader.set_validation(opts);
        }
    }
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if opts.is_none() {
            return;
        }
        self.file.set_validation(opts);
    }
}

impl MoovIoAchFile {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        self.validate_opts = opts;
        self.header.set_validation(self.validate_opts.clone());
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        self.validate_opts = opts;
    }
}
