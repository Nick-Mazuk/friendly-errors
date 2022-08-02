use crate::FriendlyError;

impl FriendlyError {
    pub fn print_file_url(&mut self) {
        if self.data.file_path.is_none()
            || self.data.line_number.is_none()
            || self.data.column_start.is_none()
        {
            return;
        }
        self.add_empty_line();
        if let Some(file_path) = &self.data.file_path {
            self.output.push_str(file_path);
            self.output.push(':');
        }
        if let Some(line_number) = &self.data.line_number {
            self.output.push_str(&line_number.to_string());
            self.output.push(':');
        }
        if let Some(column_start) = &self.data.column_start {
            self.output.push_str(&column_start.to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_append_doc_url() {
        let mut error = FriendlyError::new()
            .file_path("hello/world")
            .line_number(24)
            .column_start(42);
        error.print_file_url();
        assert_eq!(error.output, "hello/world:24:42");
    }

    #[test]
    fn test_append_doc_url_with_output() {
        let mut error = FriendlyError::new()
            .file_path("hello/world")
            .line_number(24)
            .column_start(42)
            .set_output("Error message");
        error.print_file_url();
        assert_eq!(
            error.output,
            indoc!(
                "
                Error message

                hello/world:24:42"
            )
        );
    }
}
