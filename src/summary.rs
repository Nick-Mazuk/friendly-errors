use crate::FriendlyError;

impl FriendlyError {
    pub fn print_summary(&mut self) {
        if let Some(url) = &self.data.summary {
            let url = url.clone();
            self.add_empty_line();
            self.output.push_str(&url);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_append_summary() {
        let mut error = FriendlyError::new().summary("I am the summary");
        error.print_summary();
        assert_eq!(error.output, "I am the summary");
    }

    #[test]
    fn test_append_summary_with_output() {
        let mut error = FriendlyError::new()
            .summary("I am the summary")
            .set_output("Error message");
        error.print_summary();
        assert_eq!(
            error.output,
            indoc!(
                "
                Error message

                I am the summary"
            )
        );
    }
}
