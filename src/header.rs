use crate::{ErrorKind, FriendlyError};
use colored::*;

const HEADER_LENGTH: usize = 80;

fn get_label(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::Error => "Error".to_string(),
        ErrorKind::Warning => "Warning".to_string(),
        ErrorKind::Improvement => "Improvement".to_string(),
        ErrorKind::CodeStyle => "Code style".to_string(),
    }
}

fn colorize_label(string: String, kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::Error => string.red().bold().to_string(),
        ErrorKind::Warning => string.yellow().bold().to_string(),
        ErrorKind::Improvement => string.cyan().bold().to_string(),
        ErrorKind::CodeStyle => string.cyan().bold().to_string(),
    }
}

impl FriendlyError {
    fn append_label(&mut self) -> usize {
        let mut output = String::new();
        output.push_str("--- ");
        output.push_str(&get_label(&self.data.kind));
        if let Some(code) = &self.data.error_code {
            output.push('(');
            output.push_str(code);
            output.push(')');
        }
        let length = output.len();
        self.output
            .push_str(&colorize_label(output, &self.data.kind));
        length
    }

    fn append_title(&mut self) -> usize {
        if let Some(title) = &self.data.title {
            self.output.push_str(": ");
            self.output.push_str(title);
            return title.len() + 2;
        }
        0
    }

    pub fn print_header(&mut self) {
        let mut header_length = HEADER_LENGTH;
        header_length -= self.append_label();
        header_length -= self.append_title();
        self.output.push(' ');
        header_length -= 1;
        for _ in 0..header_length {
            self.output.push('-');
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
