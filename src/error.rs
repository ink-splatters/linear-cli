use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    General,     // exit code 1
    NotFound,    // exit code 2
    Auth,        // exit code 3
    RateLimited, // exit code 4
}

impl ErrorKind {
    pub fn exit_code(self) -> u8 {
        match self {
            ErrorKind::General => 1,
            ErrorKind::NotFound => 2,
            ErrorKind::Auth => 3,
            ErrorKind::RateLimited => 4,
        }
    }

    pub fn is_retryable(self) -> bool {
        matches!(self, ErrorKind::RateLimited)
    }
}

#[derive(Debug)]
pub struct CliError {
    pub kind: ErrorKind,
    pub message: String,
    pub details: Option<Value>,
    pub retry_after: Option<u64>,
}

impl CliError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
            retry_after: None,
        }
    }

    pub fn general(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::General, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, message)
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Auth, message)
    }

    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::RateLimited, message)
    }

    pub fn code(&self) -> u8 {
        self.kind.exit_code()
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
        write!(f, "{}", self.message)?;

        // Extract and display GraphQL error messages from details if present
        if let Some(details) = &self.details {
            if let Some(errors) = details.as_array() {
                // details is a GraphQL errors array
                let messages: Vec<&str> = errors
                    .iter()
                    .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                    .collect();
                if !messages.is_empty() {
                    write!(f, ": {}", messages.join("; "))?;
                }
            } else if let Some(message) = details.get("message").and_then(|m| m.as_str()) {
                // details is an object with a message field
                write!(f, ": {}", message)?;
            } else if let Some(errors) = details.get("errors").and_then(|e| e.as_array()) {
                // details has a nested errors array
                let messages: Vec<&str> = errors
                    .iter()
                    .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                    .collect();
                if !messages.is_empty() {
                    write!(f, ": {}", messages.join("; "))?;
                }
            }
        }

        Ok(())
    }
}

impl Error for CliError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_display_without_details() {
        let err = CliError::general("Simple error");
        assert_eq!(err.to_string(), "Simple error");
    }

    #[test]
    fn test_display_with_graphql_errors_array() {
        let errors = json!([
            {"message": "Field 'foo' not found"},
            {"message": "Invalid query syntax"}
        ]);
        let err = CliError::general("GraphQL error").with_details(errors);
        assert_eq!(
            err.to_string(),
            "GraphQL error: Field 'foo' not found; Invalid query syntax"
        );
    }

    #[test]
    fn test_display_with_single_graphql_error() {
        let errors = json!([{"message": "Entity not found"}]);
        let err = CliError::general("GraphQL error").with_details(errors);
        assert_eq!(err.to_string(), "GraphQL error: Entity not found");
    }

    #[test]
    fn test_display_with_object_message() {
        let details = json!({"message": "Rate limit exceeded", "code": 429});
        let err = CliError::rate_limited("API error").with_details(details);
        assert_eq!(err.to_string(), "API error: Rate limit exceeded");
    }

    #[test]
    fn test_display_with_nested_errors_array() {
        let details = json!({
            "errors": [
                {"message": "Permission denied"},
                {"message": "Insufficient scope"}
            ]
        });
        let err = CliError::auth("Auth error").with_details(details);
        assert_eq!(
            err.to_string(),
            "Auth error: Permission denied; Insufficient scope"
        );
    }

    #[test]
    fn test_display_with_empty_errors_array() {
        let errors = json!([]);
        let err = CliError::general("GraphQL error").with_details(errors);
        assert_eq!(err.to_string(), "GraphQL error");
    }

    #[test]
    fn test_display_with_errors_missing_message() {
        let errors = json!([{"code": 123}, {"extensions": {}}]);
        let err = CliError::general("GraphQL error").with_details(errors);
        assert_eq!(err.to_string(), "GraphQL error");
    }

    #[test]
    fn test_error_kind_exit_codes() {
        assert_eq!(ErrorKind::General.exit_code(), 1);
        assert_eq!(ErrorKind::NotFound.exit_code(), 2);
        assert_eq!(ErrorKind::Auth.exit_code(), 3);
        assert_eq!(ErrorKind::RateLimited.exit_code(), 4);
    }

    #[test]
    fn test_error_kind_retryable() {
        assert!(!ErrorKind::General.is_retryable());
        assert!(!ErrorKind::NotFound.is_retryable());
        assert!(!ErrorKind::Auth.is_retryable());
        assert!(ErrorKind::RateLimited.is_retryable());
    }

    #[test]
    fn test_convenience_constructors() {
        let err = CliError::not_found("Issue not found");
        assert_eq!(err.code(), 2);
        assert_eq!(err.kind, ErrorKind::NotFound);
    }
}
