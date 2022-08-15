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
    file_path: Option<String>,
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

fn get_line_number_string(line_number: usize, indent: usize) -> String {
    let number = line_number.to_string();
    let mut output = " ".repeat(indent - 1 - number.len());
    output.push_str(&number);
    output.push_str(" | ");
    output
}

fn get_blank_line_prefix(indent: usize) -> String {
    let mut output = " ".repeat(indent);
    output.push_str("| ");
    output
}

impl FriendlyCodeSnippet {
    pub fn new(file_contents: String) -> Self {
        FriendlyCodeSnippet {
            file_contents,
            file_path: None,
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
        self.file_path = Some(file_path.into());
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

    pub(crate) fn build_file_url(&self) -> String {
        let mut output = " ".repeat(self.indent_size.unwrap());
        let mut has_contents = false;
        if let Some(file_path) = &self.file_path {
            output.push_str(file_path);
            has_contents = true;
        }
        if let Some(line_start) = self.line_start {
            if has_contents {
                output.push(':');
            }
            output.push_str(&line_start.to_string());
            has_contents = true;
        }
        if let Some(index_start) = self.index_start {
            if has_contents {
                output.push(':');
            }
            output.push_str(&index_start.to_string());
            has_contents = true;
        }
        if !has_contents {
            return String::new();
        }
        output.push('\n');
        output
    }

    pub(crate) fn build_caption(&self) -> String {
        if let Some(caption) = &self.caption {
            let mut output = " ".repeat(self.indent_size.unwrap() - 2);
            output.push_str("--> ");
            output.push_str(caption);
            output.push('\n');
            return output;
        }
        String::new()
    }

    #[cfg(test)]
    pub(crate) fn set_indent_size(mut self, indent_size: usize) -> Self {
        self.indent_size = Ok(indent_size);
        self
    }

    pub(crate) fn build(mut self) -> Result<String, FriendlyCodeSnippetError> {
        self.calc_line_start_start_index();
        self.calc_line_end_start_index();
        self.validate_inputs()?;
        self.calc_indent_size();
        let mut output = String::new();
        output.push_str(&self.build_file_url());
        output.push_str(&self.build_caption());
        Ok(output)
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

        let mut friendly_code_snippet = FriendlyCodeSnippet::new(code).line_start(2).line_end(1);
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

    #[test]
    fn get_line_number_string_test() {
        assert_eq!(get_line_number_string(1, 4), "  1 | ");
        assert_eq!(get_line_number_string(2, 4), "  2 | ");
        assert_eq!(get_line_number_string(20, 4), " 20 | ");
        assert_eq!(get_line_number_string(200, 4), "200 | ");
        assert_eq!(get_line_number_string(200, 5), " 200 | ");
    }

    #[test]
    fn get_blank_line_prefix_test() {
        assert_eq!(get_blank_line_prefix(4), "    | ");
        assert_eq!(get_blank_line_prefix(5), "     | ");
        assert_eq!(
            get_blank_line_prefix(7).len(),
            get_line_number_string(1, 7).len()
        );
    }

    #[test]
    fn build_file_url_test() {
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .set_indent_size(4)
                .build_file_url(),
            ""
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .index_start(4)
                .set_indent_size(4)
                .build_file_url(),
            "    4\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .line_start(24)
                .index_start(4)
                .set_indent_size(4)
                .build_file_url(),
            "    24:4\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .line_start(24)
                .set_indent_size(4)
                .build_file_url(),
            "    24\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .set_file_path("hello.rs")
                .set_indent_size(4)
                .build_file_url(),
            "    hello.rs\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .set_file_path("hello.rs")
                .line_start(24)
                .index_start(4)
                .set_indent_size(4)
                .build_file_url(),
            "    hello.rs:24:4\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .set_file_path("hello.rs")
                .line_start(24)
                .index_start(4)
                .set_indent_size(8)
                .build_file_url(),
            "        hello.rs:24:4\n"
        );
    }

    #[test]
    fn build_caption_test() {
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .set_indent_size(4)
                .build_caption(),
            ""
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .caption("hello world")
                .set_indent_size(4)
                .build_caption(),
            "  --> hello world\n"
        );
        assert_eq!(
            FriendlyCodeSnippet::new(String::new())
                .caption("hello world")
                .set_indent_size(8)
                .build_caption(),
            "      --> hello world\n"
        );
    }
}
