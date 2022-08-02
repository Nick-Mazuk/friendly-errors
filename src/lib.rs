pub use code_snippet::CodeSnippet;

mod code_snippet;
mod doc_url;
mod header;

pub enum ErrorKind {
    Error,
    Warning,
    Improvement,
    CodeStyle,
}

struct ErrorData {
    code_snippets: Vec<CodeSnippet>,
    doc_url: Option<String>,
    error_code: Option<String>,
    kind: ErrorKind,
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
                    code_snippets: Vec::new(),
                    doc_url: None,
                    error_code: None,
                    kind: ErrorKind::Error,
                    title: None,
                }
            },
            output: String::new(),
        }
    }

    pub fn add_code_snippet(mut self, code_snippet: CodeSnippet) -> Self {
        self.data.code_snippets.push(code_snippet);
        self
    }

    pub fn doc_url<S: Into<String>>(mut self, url: S) -> Self {
        self.data.doc_url = Some(url.into());
        self
    }

    pub fn error_code<S: Into<String>>(mut self, code: S) -> Self {
        self.data.error_code = Some(code.into());
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
