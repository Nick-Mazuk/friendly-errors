use crate::FriendlyError;

impl FriendlyError {
    pub fn print_description(&mut self) {
        if let Some(url) = &self.data.description {
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
    fn test_append_description() {
        let mut error = FriendlyError::new().description("I am the description");
        error.print_description();
        assert_eq!(error.output, "I am the description");
    }

    #[test]
    fn test_append_description_with_output() {
        let mut error = FriendlyError::new()
            .description("I am the description")
            .set_output("Error message");
        error.print_description();
        assert_eq!(
            error.output,
            indoc!(
                "
                Error message

                I am the description"
            )
        );
    }
}
