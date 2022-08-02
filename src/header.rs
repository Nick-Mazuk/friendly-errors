use crate::{ErrorKind, FriendlyError};
use colored::*;

const HEADER_LENGTH: usize = 80;

impl FriendlyError {
    fn gen_error(&mut self) -> (String, usize) {
        let mut output = String::new();
        output.push_str("--- Error");
        if let Some(code) = &self.data.error_code {
            output.push('(');
            output.push_str(code);
            output.push(')');
        }
        let length = output.len();
        (output.red().bold().to_string(), length)
    }

    fn gen_warning(&mut self) -> (String, usize) {
        let mut output = String::new();
        output.push_str("--- Warning");
        if let Some(code) = &self.data.error_code {
            output.push('(');
            output.push_str(code);
            output.push(')');
        }
        let length = output.len();
        (output.yellow().bold().to_string(), length)
    }

    pub fn print_header(&mut self) {
        let mut header_length = 0;
        let (type_string, length) = match self.data.kind {
            ErrorKind::Error => self.gen_error(),
            ErrorKind::Warning => self.gen_warning(),
        };
        header_length += length;
        self.output.push_str(&type_string);
        if let Some(title) = &self.data.title {
            self.output.push_str(": ");
            self.output.push_str(title);
            header_length += title.len() + 2;
        }
        self.output.push(' ');
        header_length += 1;
        while header_length < HEADER_LENGTH {
            self.output.push('-');
            header_length += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_base_case() {
        colored::control::set_override(false);
        let mut error = FriendlyError::new();
        error.print_header();
        assert_eq!(
            error.output,
            "--- Error ----------------------------------------------------------------------"
        );
    }

    #[test]
    fn header_with_title() {
        colored::control::set_override(false);
        let mut error = FriendlyError::new().title("Error message");
        error.print_header();
        assert_eq!(
            error.output,
            "--- Error: Error message -------------------------------------------------------"
        );
    }

    #[test]
    fn header_with_code() {
        colored::control::set_override(false);
        let mut error = FriendlyError::new().error_code("E123");
        error.print_header();
        assert_eq!(
            error.output,
            "--- Error(E123) ----------------------------------------------------------------"
        );
    }

    #[test]
    fn header_with_title_and_code() {
        colored::control::set_override(false);
        let mut error = FriendlyError::new()
            .title("Error message")
            .error_code("E123");
        error.print_header();
        assert_eq!(
            error.output,
            "--- Error(E123): Error message -------------------------------------------------"
        );
    }
}
