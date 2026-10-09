//! Core library for the generated workspace.

use thiserror::Error;

/// Errors returned by core operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// Returned when the input is empty.
    #[error("input must not be empty")]
    EmptyInput,
}

/// Build a greeting for `name`.
///
/// # Errors
///
/// Returns [`CoreError::EmptyInput`] when `name` is empty.
///
/// # Examples
///
/// ```
/// use {{crate_name}}_core::greet;
/// assert_eq!(greet("Rust")?, "Hello, Rust!");
/// # Ok::<(), {{crate_name}}_core::CoreError>(())
/// ```
pub fn greet(name: &str) -> Result<String, CoreError> {
    if name.is_empty() {
        return Err(CoreError::EmptyInput);
    }

    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_greeting() {
        assert_eq!(greet("Rust").as_deref(), Ok("Hello, Rust!"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(greet(""), Err(CoreError::EmptyInput));
    }
}
