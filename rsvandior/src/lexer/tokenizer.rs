use std::ffi::CStr;

use crate::token::{CodeSourceLocation, RawToken, Token, TokenType};
use logos::{Lexer, Logos};

#[repr(C)]
#[allow(dead_code)]
pub struct Tokenizer<'a> {
    input: &'a str,
    file_name: &'a str,
    line: usize,
    column: usize,
    lexer: Lexer<'a, RawToken>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(file_name: &'a str, input: &'a str) -> Self {
        Self {
            file_name: file_name,
            input,
            line: 0,
            column: 0,
            lexer: RawToken::lexer(input),
        }
    }

    fn get_line_and_column(&mut self) -> (usize, usize) {
        let byte_index = self.lexer.span().start;
        let lines: Vec<&str> = self.input[..byte_index].split('\n').collect();
        let line_number = lines.len();
        let column_number = lines.last().unwrap_or(&"").chars().count();
        (line_number, column_number)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(raw_token) = self.lexer.next() {
            //let span = lexer.span();
            let value = self.lexer.clone().slice();
            let (line, column) = self.get_line_and_column();
            let source_location = CodeSourceLocation::new(self.file_name.to_string(), line, column);
            let (token_type, remapped_value) =
                TokenType::map_raw_token_type(raw_token.clone().unwrap(), value.to_string());
            tokens.push(Token::new(
                token_type,
                remapped_value.to_string(),
                source_location,
            ));
            self.line = line;
            self.column = column + 1;
        }
        tokens.push(Token::new(
            TokenType::Eoft,
            "".to_string(),
            CodeSourceLocation::new(self.file_name.to_string(), self.line, self.column),
        ));
        tokens
    }
}

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TokenDynamicArray {
    ptr: *mut Token, // Puntatore all'array allocato
    capacity: usize, // Capacità totale dell'array
    size: usize,     // Numero di elementi nell'array
}

impl TokenDynamicArray {
    pub fn new() -> Self {
        TokenDynamicArray {
            ptr: ptr::null_mut(),
            capacity: 0,
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<Token>(capacity).expect("Layout creation failed");
        let ptr = unsafe { alloc(layout) as *mut Token };
        TokenDynamicArray {
            ptr,
            capacity,
            size: 0,
        }
    }

    pub fn push(&mut self, item: Token) {
        if self.size == self.capacity {
            self.resize();
        }
        unsafe {
            ptr::write(self.ptr.add(self.size), item);
        }
        self.size += 1;
    }

    fn resize(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let new_layout = Layout::array::<Token>(new_capacity).expect("Layout creation failed");
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) as *mut Token }
        } else {
            let old_layout = Layout::array::<Token>(self.capacity).expect("Layout creation failed");
            unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut Token }
        };
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }

    pub fn pop(&mut self) -> Option<Token> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe { Some(ptr::read(self.ptr.add(self.size))) }
        }
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        if index < self.size {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, item: Token) {
        if index < self.size {
            unsafe {
                ptr::write(self.ptr.add(index), item);
            }
        } else {
            panic!("Index out of bounds");
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl Drop for TokenDynamicArray {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.clear();
            let layout = Layout::array::<Token>(self.capacity).expect("Layout creation failed");
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn tokenize_w(
    file_name: *const std::os::raw::c_char,
    input: *const std::os::raw::c_char,
) -> TokenDynamicArray {
    let c_str1 = unsafe { CStr::from_ptr(file_name) };
    let c_str2 = unsafe { CStr::from_ptr(input) };
    let file_name_str = c_str1.to_str().unwrap_or("Unknown");
    let input_str = c_str2.to_str().unwrap_or("Unknown");
    let mut tokenizer = Tokenizer::new(file_name_str, input_str);

    let tolens_vec = tokenizer.tokenize();
    let mut tokens = TokenDynamicArray::with_capacity(tolens_vec.len());
    tolens_vec.into_iter().for_each(|token| {
        tokens.push(token);
    });
    tokens
    //Box::new(tokenizer.tokenize())
}
