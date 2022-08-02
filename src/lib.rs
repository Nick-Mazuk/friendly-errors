mod doc_url;
mod header;

pub enum ErrorKind {
    Error,
    Warning,
}

struct ErrorData {
    code: Option<String>,
    kind: ErrorKind,
    title: Option<String>,
    doc_url: Option<String>,
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
                    doc_url: None,
                    kind: ErrorKind::Error,
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

    pub fn doc_url<S: Into<String>>(mut self, url: S) -> Self {
        self.data.doc_url = Some(url.into());
        self
    }

    pub fn kind(mut self, kind: ErrorKind) -> Self {
        self.data.kind = kind;
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.data.title = Some(title.into());
        self
    }

    pub fn build(mut self) -> String {
        self.print_header();
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
