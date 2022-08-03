pub enum HighlightKind {
    Error,
    Warning,
    Info,
}

pub struct FriendlyCodeSnippet {
    file_contents: String,
    file_path: String,
    index_start: Option<usize>,
    index_end: Option<usize>,
    line_start: Option<usize>,
    line_end: Option<usize>,
    column_start: Option<usize>,
    column_end: Option<usize>,
    kind: HighlightKind,
    caption: Option<String>,
}

impl FriendlyCodeSnippet {
    pub fn new(file_path: String, file_contents: String) -> Self {
        FriendlyCodeSnippet {
            file_contents,
            file_path,
            index_start: None,
            index_end: None,
            line_start: None,
            line_end: None,
            column_start: None,
            column_end: None,
            kind: HighlightKind::Error,
            caption: None,
        }
    }

    pub fn index_start(mut self, index_start: usize) -> Self {
        self.index_start = Some(index_start);
        self
    }

    pub fn index_end(mut self, index_end: usize) -> Self {
        self.index_end = Some(index_end);
        self
    }

    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = Some(line_start);
        self
    }

    pub fn line_end(mut self, line_end: usize) -> Self {
        self.line_end = Some(line_end);
        self
    }

    pub fn column_start(mut self, column_start: usize) -> Self {
        self.column_start = Some(column_start);
        self
    }

    pub fn column_end(mut self, column_end: usize) -> Self {
        self.column_end = Some(column_end);
        self
    }

    pub fn kind(mut self, kind: HighlightKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub(crate) fn build(mut self) -> String {
        String::new()
    }
}
