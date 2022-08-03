pub use code_snippet::FriendlyCodeSnippet;

mod code_snippet;
mod description;
mod doc_url;
mod header;
mod summary;

pub enum ErrorKind {
    Error,
    Warning,
    Improvement,
    CodeStyle,
}

struct ErrorData {
    code_snippets: Vec<FriendlyCodeSnippet>,
    description: Option<String>,
    doc_url: Option<String>,
    error_code: Option<String>,
    kind: ErrorKind,
    summary: Option<String>,
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
                    description: None,
                    doc_url: None,
                    error_code: None,
                    kind: ErrorKind::Error,
                    summary: None,
                    title: None,
                }
            },
            output: String::new(),
        }
    }

    pub fn add_code_snippet(mut self, code_snippet: FriendlyCodeSnippet) -> Self {
        self.data.code_snippets.push(code_snippet);
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.data.description = Some(description.into());
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

    pub fn summary<S: Into<String>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.into());
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.data.title = Some(title.into());
        self
    }

    pub fn build(mut self) -> String {
        self.print_header();
        self.print_summary();
        self.print_description();
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
