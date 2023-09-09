use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub const SUBJECT_STRING: &str = "Subject: ";

pub struct Email {
    pub subject: Option<String>,
    pub content: String,
}

impl Email {
    pub fn new() -> Self {
        Email {
            subject: None,
            content: String::new(),
        }
    }

    pub fn save(&self, filepath: &str) -> io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

impl Default for Email {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EmailExtractor<R: BufRead> {
    reader: R,
    separator_regex: Regex,
}

impl<R: BufRead> EmailExtractor<R> {
    /// Creates a new `EmailExtractor` with the provided `reader`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A `BufRead` object that provides the lines to be processed.
    ///
    /// # Returns
    ///
    /// A new `EmailExtractor` instance.
    pub fn new(reader: R) -> Self {
        EmailExtractor {
            reader,
            // From https://github.com/giomba/mbox2eml/blob/master/src/mbox2eml.cpp#L27
            separator_regex: Regex::new(r"^From .*@.*\..*\s+[A-Z][a-z]{2} [A-Z][a-z]{2} \d{1,2} \d{1,2}:\d{1,2}:\d{1,2} \d{4}$").unwrap(),
        }
    }

    /// Extracts emails from the provided `reader` based on the separator regex.
    ///
    /// # Returns
    ///
    /// A vector of `Email` objects containing the extracted emails.
    pub fn extract_emails(&mut self) -> Vec<Email> {
        let mut emails = Vec::new();
        let mut current_email = Email::new();

        let mut line = String::new();
        while self.reader.read_line(&mut line).unwrap() > 0 {
            line = line.trim_end().to_string();
            if self.separator_regex.is_match(&line) {
                #[cfg(debug_assertions)]
                dbg!("Found separator: {}", line.clone());

                if !current_email.content.is_empty() {
                    emails.push(current_email);
                    current_email = Email::new();
                }
            } else if let Some(subject) = line.strip_prefix(SUBJECT_STRING) {
                #[cfg(debug_assertions)]
                dbg!("Found subject: {}", subject);

                current_email.subject = Some(subject.trim().to_string());
            }
            current_email.content.push_str(&line);
            current_email.content.push('\n');
            line.clear(); // clear the buffer for the next iteration
        }

        // Save the last email if there is one.
        if !current_email.content.is_empty() {
            emails.push(current_email);
        }

        emails
    }
}
