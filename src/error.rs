use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CliError {
    pub code: u8,
    pub message: String,
    pub details: Option<Value>,
    pub retry_after: Option<u64>,
}

impl CliError {
    pub fn new(code: u8, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            retry_after: None,
        }
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_retry_after(mut self, retry_after: Option<u64>) -> Self {
        self.retry_after = retry_after;
        self
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CliError {}
