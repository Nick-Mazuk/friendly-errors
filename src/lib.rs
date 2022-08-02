mod doc_url;
mod file_url;
mod header;

pub enum ErrorKind {
    Error,
    Warning,
}

struct ErrorData {
    code: Option<String>,
    column_start: Option<u32>,
    doc_url: Option<String>,
    file_path: Option<String>,
    kind: ErrorKind,
    line_number: Option<u32>,
    title: Option<String>,
}

pub struct FriendlyError {
    data: ErrorData,
    pub(crate) output: String,
}

impl FriendlyError {
    pub fn new() -> Self {
        FriendlyError {
            data: {
                ErrorData {
                    code: None,
                    column_start: None,
                    doc_url: None,
                    file_path: None,
                    kind: ErrorKind::Error,
                    line_number: None,
                    title: None,
                }
            },
            output: String::new(),
        }
    }

    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.data.code = Some(code.into());
        self
    }

    pub fn column_start(mut self, column_start: u32) -> Self {
        self.data.column_start = Some(column_start);
        self
    }

    pub fn doc_url<S: Into<String>>(mut self, url: S) -> Self {
        self.data.doc_url = Some(url.into());
        self
    }

    pub fn file_path<S: Into<String>>(mut self, file_path: S) -> Self {
        self.data.file_path = Some(file_path.into());
        self
    }

    pub fn kind(mut self, kind: ErrorKind) -> Self {
        self.data.kind = kind;
        self
    }

    pub fn line_number(mut self, line_number: u32) -> Self {
        self.data.line_number = Some(line_number);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.data.title = Some(title.into());
        self
    }

    pub fn build(mut self) -> String {
        self.print_header();
        self.print_file_url();
        self.print_doc_url();
        self.output
    }

    #[cfg(test)]
    pub(crate) fn set_output<S: Into<String>>(mut self, output: S) -> Self {
        self.output = output.into();
        self
    }

    pub(crate) fn add_empty_line(&mut self) {
        if !self.output.is_empty() {
            self.output.push('\n');
            self.output.push('\n');
        }
    }
}

impl Default for FriendlyError {
    fn default() -> Self {
        Self::new()
    }
}
