use super::Token;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TokenList {
    pub file_name: String,
    pub tokens: Vec<Token>,
}

impl TokenList {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            tokens: Vec::new(),
        }
    }
}