use std::cmp::max;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HighlightKind {
    Error,
    Warning,
    Info,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum CalculatedFieldError {
    NotCalculated,
    Invalid,
}

type CalculatedFieldResult<T> = Result<T, CalculatedFieldError>;

#[derive(PartialEq, Debug, Clone)]
pub struct FriendlyCodeSnippet {
    file_contents: String,
    file_path: String,
    index_start: Option<usize>,
    index_end: Option<usize>,
    line_start: Option<usize>,
    line_end: Option<usize>,
    kind: HighlightKind,
    caption: Option<String>,

    // private fields
    line_start_start_index: CalculatedFieldResult<usize>,
    line_end_start_index: CalculatedFieldResult<usize>,
    indent_size: CalculatedFieldResult<usize>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FriendlyCodeSnippetError {
    InvalidStartPosition,
    InvalidEndPosition,
    MissingStartPosition,
    MissingEndPosition,
}

fn get_digit_count(mut number: usize) -> usize {
    let mut digits = 1;
    while number >= 10 {
        digits += 1;
        number /= 10;
    }
    digits
}

impl FriendlyCodeSnippet {
    pub fn new(file_contents: String) -> Self {
        FriendlyCodeSnippet {
            file_contents,
            file_path: String::new(),
            index_start: None,
            index_end: None,
            line_start: None,
            line_end: None,
            kind: HighlightKind::Error,
            caption: None,

            // private fields
            line_start_start_index: Err(CalculatedFieldError::NotCalculated),
            line_end_start_index: Err(CalculatedFieldError::NotCalculated),
            indent_size: Err(CalculatedFieldError::NotCalculated),
        }
    }

    pub fn set_file_path<S: Into<String>>(mut self, file_path: S) -> Self {
        self.file_path = file_path.into();
        self
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

    pub fn kind(mut self, kind: HighlightKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub(crate) fn build(mut self) -> Result<String, FriendlyCodeSnippetError> {
        self.calc_line_start_start_index();
        self.calc_line_end_start_index();
        self.validate_inputs()?;
        Ok(String::new())
    }

    pub(crate) fn calc_line_start_start_index(&mut self) {
        match self.line_start {
            Some(line) => {
                let mut line_count = 1;
                for (index, char) in self.file_contents.chars().enumerate() {
                    if line_count == line {
                        self.line_start_start_index = Ok(index);
                        return;
                    }
                    if char == '\n' {
                        line_count += 1;
                    }
                }
                self.line_start_start_index = Err(CalculatedFieldError::Invalid);
            }
            None => {
                self.line_start_start_index = Err(CalculatedFieldError::Invalid);
            }
        }
    }

    pub(crate) fn calc_line_end_start_index(&mut self) {
        match self.line_end {
            Some(line) => {
                let mut line_count = 1;
                for (index, char) in self.file_contents.chars().enumerate() {
                    if line_count == line {
                        self.line_end_start_index = Ok(index);
                        return;
                    }
                    if char == '\n' {
                        line_count += 1;
                    }
                }
                self.line_end_start_index = Err(CalculatedFieldError::Invalid);
            }
            None => {
                self.line_end_start_index = Err(CalculatedFieldError::Invalid);
            }
        }
    }

    pub(crate) fn validate_inputs(&self) -> Result<bool, FriendlyCodeSnippetError> {
        if self.line_start.is_none() && self.index_start.is_none() {
            return Err(FriendlyCodeSnippetError::MissingStartPosition);
        }
        if self.line_end.is_none() && self.index_end.is_none() {
            return Err(FriendlyCodeSnippetError::MissingEndPosition);
        }
        match self.line_start_start_index {
            Err(CalculatedFieldError::Invalid) => {
                return Err(FriendlyCodeSnippetError::InvalidStartPosition)
            }
            Err(CalculatedFieldError::NotCalculated) => {
                panic!("line_start_start_index must be calculated before inputs are validated")
            }
            Ok(_) => {}
        }
        match self.line_end_start_index {
            Err(CalculatedFieldError::Invalid) => {
                return Err(FriendlyCodeSnippetError::InvalidEndPosition)
            }
            Err(CalculatedFieldError::NotCalculated) => {
                panic!("line_end_start_index must be calculated before inputs are validated")
            }
            Ok(_) => {}
        }
        if self.line_start_start_index.unwrap() > self.line_end_start_index.unwrap() {
            return Err(FriendlyCodeSnippetError::InvalidEndPosition);
        }
        Ok(true)
    }

    pub(crate) fn calc_indent_size(&mut self) {
        let longest_line_number = max(
            self.line_start_start_index.unwrap(),
            self.line_end_start_index.unwrap(),
        );
        let default_indent_size = 4;
        self.indent_size = Ok(max(
            get_digit_count(longest_line_number) + 1,
            default_indent_size,
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_line_start_start_index_test() {
        let code = "\nfn main() {\n    println!(\"Hello, world!\");\n}\n".to_string();

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(1);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(friendly_code_snippet.line_start_start_index, Ok(0));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(2);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(friendly_code_snippet.line_start_start_index, Ok(1));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(3);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(friendly_code_snippet.line_start_start_index, Ok(13));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(0);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(
            friendly_code_snippet.line_start_start_index,
            Err(CalculatedFieldError::Invalid)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(100);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(
            friendly_code_snippet.line_start_start_index,
            Err(CalculatedFieldError::Invalid)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code);
        friendly_code_snippet.calc_line_start_start_index();
        assert_eq!(
            friendly_code_snippet.line_start_start_index,
            Err(CalculatedFieldError::Invalid)
        );
    }

    #[test]
    fn calc_line_end_start_index_test() {
        let code = "\nfn main() {\n    println!(\"Hello, world!\");\n}\n".to_string();

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_end(1);
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(friendly_code_snippet.line_end_start_index, Ok(0));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_end(2);
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(friendly_code_snippet.line_end_start_index, Ok(1));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_end(3);
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(friendly_code_snippet.line_end_start_index, Ok(13));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_end(0);
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.line_end_start_index,
            Err(CalculatedFieldError::Invalid)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code).line_end(100);
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.line_end_start_index,
            Err(CalculatedFieldError::Invalid)
        );
    }

    #[test]
    fn validate_inputs_test() {
        let code = "\nfn main() {\n    println!(\"Hello, world!\");\n}\n".to_string();

        // everything is valid
        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone())
            .line_start(1)
            .line_end(2);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(friendly_code_snippet.validate_inputs(), Ok(true));

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_start(2);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.validate_inputs(),
            Err(FriendlyCodeSnippetError::MissingEndPosition)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone()).line_end(2);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.validate_inputs(),
            Err(FriendlyCodeSnippetError::MissingStartPosition)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone())
            .line_start(0)
            .line_end(2);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.validate_inputs(),
            Err(FriendlyCodeSnippetError::InvalidStartPosition)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone())
            .line_start(1)
            .line_end(100);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.validate_inputs(),
            Err(FriendlyCodeSnippetError::InvalidEndPosition)
        );

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code.clone())
            .line_start(2)
            .line_end(1);
        friendly_code_snippet.calc_line_start_start_index();
        friendly_code_snippet.calc_line_end_start_index();
        assert_eq!(
            friendly_code_snippet.validate_inputs(),
            Err(FriendlyCodeSnippetError::InvalidEndPosition)
        );
    }

    #[test]
    fn get_digit_count_test() {
        assert_eq!(get_digit_count(0), 1);
        assert_eq!(get_digit_count(1), 1);
        assert_eq!(get_digit_count(10), 2);
        assert_eq!(get_digit_count(100), 3);
        assert_eq!(get_digit_count(1000), 4);
        assert_eq!(get_digit_count(500), 3);
        assert_eq!(get_digit_count(99), 2);
        assert_eq!(get_digit_count(101), 3);
        assert_eq!(get_digit_count(999), 3);
        assert_eq!(get_digit_count(1001), 4);
    }
}
