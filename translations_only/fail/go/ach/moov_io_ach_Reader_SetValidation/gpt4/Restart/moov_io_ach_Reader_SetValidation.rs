
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

#[derive(Clone)]
struct MoovIoAchValidateOpts {
    // Fields as per Go code
}

impl MoovIoAchReader {
    fn set_validation(&mut self, opts: Option<Box<MoovIoAchValidateOpts>>) {
        if self.file.validate_opts.is_none() || opts.is_none() {
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
            self.header.set_validation(Some(Box::new((**opts).clone())));
        } else {
            self.header.set_validation(None);
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
