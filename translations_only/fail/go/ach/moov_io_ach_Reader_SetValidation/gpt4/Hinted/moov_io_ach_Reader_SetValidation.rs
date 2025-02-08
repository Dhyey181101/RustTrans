
struct MoovIoAchReader {
    file: MoovIoAchFile,
}

struct MoovIoAchFile {
    header: MoovIoAchFileHeader,
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchFileHeader {
    validate_opts: Option<Box<MoovIoAchValidateOpts>>,
}

struct MoovIoAchValidateOpts {
    // Fields as per Go struct
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
        if self.validate_opts.is_none() {
            return;
        }
        self.validate_opts = opts.clone();
        if let Some(ref opts) = opts {
            self.header.set_validation(Some(opts.clone()));
        }
    }
}

impl MoovIoAchFileHeader {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if self.validate_opts.is_none() {
            return;
        }
        self.validate_opts = opts;
    }
}

impl Clone for MoovIoAchValidateOpts {
    fn clone(&self) -> MoovIoAchValidateOpts {
        MoovIoAchValidateOpts {
            // Clone fields as per Go struct
        }
    }
}
