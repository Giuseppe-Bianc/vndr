use super::code_source_location::CodeSourceLocation;
use super::token_type::TokenType;
use std::ffi::{CStr, CString};
use std::fmt;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: *const std::os::raw::c_char,
    pub source_location: CodeSourceLocation,
}
#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, value: String, source_location: CodeSourceLocation) -> Self {
        Self {
            token_type,
            value: CString::new(value)
                .unwrap_or_else(|err| panic!("Tokenization failed: {}", err))
                .into_raw(),
            source_location,
        }
    }

    pub fn new_with_empty_value(
        token_type: TokenType,
        source_location: CodeSourceLocation,
    ) -> Self {
        Self {
            token_type,
            value: CString::new("")
                .unwrap_or_else(|err| panic!("Tokenization failed: {}", err))
                .into_raw(),
            source_location,
        }
    }

    pub fn is_type(&self, token_type: &TokenType) -> bool {
        &self.token_type == token_type
    }

    pub fn is_type_any_of(&self, token_types: &[TokenType]) -> bool {
        token_types.contains(&self.token_type)
    }

    pub fn value_size(&self) -> usize {
        let c_str1 = unsafe { CStr::from_ptr(self.value) };
        let value_str = c_str1.to_str().unwrap_or("Unknown");
        value_str.len()
    }

    pub fn to_compact_string(&self) -> String {
        let c_str1 = unsafe { CStr::from_ptr(self.value) };
        let value_str = c_str1.to_str().unwrap_or("Unknown");
        if value_str.is_empty() {
            format!(
                "(typ: {:#}, sl: {:#})",
                self.token_type, self.source_location
            )
        } else {
            format!(
                "(typ: {:#}, val: '{}', sl: {:#})",
                self.token_type, value_str, self.source_location
            )
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.to_compact_string())
        } else {
            let c_str1 = unsafe { CStr::from_ptr(self.value) };
            let value_str = c_str1.to_str().unwrap_or("Unknown");
            if value_str.is_empty() {
                write!(
                    f,
                    "Token(type: {:?}, sourceLocation: {})",
                    self.token_type, self.source_location
                )
            } else {
                write!(
                    f,
                    "Token(type: {:?}, value: '{}', sourceLocation: {})",
                    self.token_type, value_str, self.source_location
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::code_source_location::CodeSourceLocation;
    use crate::token::token_type::TokenType;

    #[test]
    fn test_token_new() {
        let token = Token::new(
            TokenType::Identifier,
            "test".to_string(),
            CodeSourceLocation::new("test".to_string(), 1, 1),
        );
        assert_eq!(token.token_type, TokenType::Identifier);
        let c_str1 = unsafe { CStr::from_ptr(token.value) }
            .to_str()
            .unwrap_or("Unknown");
        assert_eq!(c_str1, "test");
        assert_eq!(
            unsafe { CStr::from_ptr(token.source_location.file_name) }
                .to_str()
                .unwrap_or("Unknown"),
            "test"
        );
        assert_eq!(token.source_location.line, 1);
        assert_eq!(token.source_location.column, 1);
    }

    #[test]
    fn test_token_new_with_empty_value() {
        let token = Token::new_with_empty_value(
            TokenType::Identifier,
            CodeSourceLocation::new("test".to_string(), 1, 1),
        );
        assert_eq!(token.token_type, TokenType::Identifier);
        let c_str1 = unsafe { CStr::from_ptr(token.value) }
            .to_str()
            .unwrap_or("Unknown");
        assert_eq!(c_str1, "");
        assert_eq!(
            unsafe { CStr::from_ptr(token.source_location.file_name) }
                .to_str()
                .unwrap_or("Unknown"),
            "test"
        );
        assert_eq!(token.source_location.line, 1);
        assert_eq!(token.source_location.column, 1);
    }

    #[test]
    fn test_token_is_type() {
        let token = Token::new(
            TokenType::Identifier,
            "test".to_string(),
            CodeSourceLocation::new("test".to_string(), 1, 1),
        );
        assert!(token.is_type(&TokenType::Identifier));
        assert!(!token.is_type(&TokenType::Integer));
    }

    #[test]
    fn test_token_is_type_any_of() {
        let token = Token::new(
            TokenType::Identifier,
            "test".to_string(),
            CodeSourceLocation::new("test".to_string(), 1, 1),
        );
        assert!(token.is_type_any_of(&[TokenType::Identifier, TokenType::Integer]));
        assert!(!token.is_type_any_of(&[TokenType::Integer]));
    }
}
