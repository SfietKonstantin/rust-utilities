use std::ffi::{CStr, CString};

/// Convenient trait to work with `CString`
///
/// This trait is implemented by everything that resemble a string
/// to convert conveniently to a `CString`.
pub trait StringExt {
    /// Convenient method to create a `CString` from a string
    ///
    /// Replaces \0 with the `U+FFFD REPLACEMENT CHARACTER`.
    fn to_cstring_lossy(&self) -> CString;
}

impl<T> StringExt for T
where
    T: AsRef<str>,
{
    fn to_cstring_lossy(&self) -> CString {
        let string = self
            .as_ref()
            .chars()
            .map(replace_zero_with_replacement_char)
            .collect::<String>();
        CString::new(string).unwrap_or_default()
    }
}

fn replace_zero_with_replacement_char(c: char) -> char {
    if c == '\0' {
        char::REPLACEMENT_CHARACTER
    } else {
        c
    }
}

/// Convenient trait to work with `CString`
///
/// This trait is implemented by `CStr` and `CString` to convert conveniently
/// to `String`.
pub trait CStringExt {
    /// Convenient method to create a string from a `CStr`
    /// or `CString`.
    ///
    /// Replaces non-UTF8 sequences with the `U+FFFD REPLACEMENT CHARACTER`.
    fn to_string_lossy(&self) -> String;
}

impl<T> CStringExt for T
where
    T: AsRef<CStr>,
{
    fn to_string_lossy(&self) -> String {
        self.as_ref().to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_string() {
        let input_string = "test";
        let cstring = input_string.to_cstring_lossy();
        let string = cstring.to_string_lossy();
        assert_eq!(string, input_string);
    }

    #[test]
    fn test_string_with_zero() {
        let input_string = "test\0test2";
        let cstring = input_string.to_cstring_lossy();
        let string = cstring.to_string_lossy();
        assert_eq!(string, format!("test{}test2", char::REPLACEMENT_CHARACTER));
    }
}
