use crate::{FriendlyError, FriendlyErrorError};

impl FriendlyError {
    pub fn print_code_snippets(&mut self) -> Result<bool, FriendlyErrorError> {
        let snippets = self.data.code_snippets.clone();
        for snippet in snippets.iter() {
            let output = snippet.clone().build();
            match output {
                Ok(output) => {
                    self.add_empty_line();
                    self.output.push_str(&output);
                }
                Err(err) => return Err(FriendlyErrorError::CodeSnippetError(err)),
            }
        }
        Ok(true)
    }
}
