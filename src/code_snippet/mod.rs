pub enum HighlightKind {
    Error,
    Warning,
    Info,
}

pub struct CodeSnippet {
    file_contents: String,
    file_path: String,
    line_number: u32,
    index_start: usize,
    index_end: usize,
    kind: HighlightKind,
    caption: Option<String>,
}
