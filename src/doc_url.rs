use crate::FriendlyError;

impl FriendlyError {
    pub fn print_doc_url(&mut self) {
        if let Some(url) = &self.data.doc_url {
            let url = url.clone();
            self.add_empty_line();
            self.output.push_str("To learn more, read the docs at ");
            self.output.push_str(&url);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_append_doc_url() {
        let mut error = FriendlyError::new().doc_url("https://example.com/");
        error.print_doc_url();
        assert_eq!(
            error.output,
            "To learn more, read the docs at https://example.com/"
        );
    }

    #[test]
    fn test_append_doc_url_with_output() {
        let mut error = FriendlyError::new()
            .doc_url("https://example.com/")
            .set_output("Error message");
        error.print_doc_url();
        assert_eq!(
            error.output,
            indoc!(
                "
                Error message

                To learn more, read the docs at https://example.com/"
            )
        );
    }
}
