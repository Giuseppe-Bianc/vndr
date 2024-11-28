use std::{
    ffi::{CStr, CString},
    fmt,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CodeSourceLocation {
    pub file_name: *const std::os::raw::c_char,
    pub line: usize,
    pub column: usize,
}

impl CodeSourceLocation {
    pub fn new(file_name: String, line: usize, column: usize) -> Self {
        Self {
            file_name: CString::new(file_name).unwrap().into_raw(),
            line,
            column,
        }
    }

    pub fn to_compact_string(&self) -> String {
        let c_str1 = unsafe { CStr::from_ptr(self.file_name) };
        let file_name_str = c_str1.to_str().unwrap_or("Unknown");
        format!(
            "(fn: {}, ln: {}, cln: {})",
            file_name_str, self.line, self.column
        )
    }
}

impl fmt::Display for CodeSourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.to_compact_string())
        } else {
            let c_str1 = unsafe { CStr::from_ptr(self.file_name) };
            let file_name_str = c_str1.to_str().unwrap_or("Unknown");
            // Usa il formato standard.
            write!(
                f,
                "(file: {},line: {}, column: {})",
                file_name_str, self.line, self.column
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_code_source_location() {
        let location = CodeSourceLocation::new("main.rs".to_string(), 10, 20);
        let c_str = unsafe { CStr::from_ptr(location.file_name) };
        assert_eq!(c_str.to_str().unwrap(), "main.rs");
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 20);
    }

    #[test]
    fn test_to_compact_string() {
        let location = CodeSourceLocation::new("lib.rs".to_string(), 5, 15);
        let compact_str = location.to_compact_string();
        assert_eq!(compact_str, "(fn: lib.rs, ln: 5, cln: 15)");
    }

    #[test]
    fn test_display() {
        let location = CodeSourceLocation::new("mod.rs".to_string(), 3, 8);
        let display_str = format!("{}", location);
        assert_eq!(display_str, "(file: mod.rs,line: 3, column: 8)");

        let compact_display_str = format!("{:#}", location);
        assert_eq!(compact_display_str, "(fn: mod.rs, ln: 3, cln: 8)");
    }
}
