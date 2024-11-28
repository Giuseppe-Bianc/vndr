use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum RawToken {
    // ASCII identifiers (including underscores)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    IdentifierAscii,

    // Unicode identifiers (including underscores)
    #[regex(r"[\p{Letter}\p{Mark}_][\p{Letter}\p{Mark}\p{Number}_]*", priority = 1)]
    IdentifierUnicode,

    // Numbers: integers, floats, scientific notation, complex numbers
    #[regex(r"(\d*\.\d+|\d+\.|\d+)([eE][+-]?\d+)?[if]*", priority = 4)]
    Number,

    // Binary numbers (e.g., #b1010, #b1101)
    #[regex(r"##[01]+", priority = 2)]
    Binary,

    // Hexadecimal numbers (e.g., #ff, #7f), excluding #b
    #[regex(r"#([0-9a-fA-F]+)", priority = 3)]
    Hexadecimal,

    // Octal numbers (e.g., #o23, #o24)
    #[regex(r"#o([0-7]+)", priority = 2)]
    Octal,

    // Whitespace (including Unicode spaces)
    #[regex(
        r"[ \t\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+",
        logos::skip
    )]
    Whitespace,

    #[regex(r"//[^\n]*")] // Skip inline comments
    SingleLineComment,
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")] // Skip multi-line comments
    MultiLineComment,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    LESS,
    #[token(">")]
    GREATER,
    #[token("!")]
    NOT,
    #[token("^")]
    XOR,
    #[token("%")]
    PERCENT,
    #[token("|")]
    OR,
    #[token("&")]
    AND,
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("+=")]
    PlusEqual,
    #[token("-?")]
    MinusEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    Greaterequal,
    #[token("!=")]
    NotEqual,
    #[token("^=")]
    XorEqual,
    #[token("%=")]
    PercentEqual,

    /*#[token("%")]
    PERCENT,*/
    #[token("||")]
    OrOr,
    #[token("&&")]
    AndAnd,

    // Parentheses
    #[token("(")]
    OpenParentesis,
    #[token(")")]
    CloseParentesis,

    // Square brackets
    #[token("[")]
    OpenSQParentesis,
    #[token("]")]
    CloseSQParentesis,

    // Curly brackets
    #[token("{")]
    OpenCurParentesis,
    #[token("}")]
    CloseCurParentesis,

    #[regex(r"true|false", priority = 5)]
    BOOLEAN,

    // Strings: Matches double-quoted strings, including escape sequences
    #[regex(r#""([^"\\]|\\.)*""#)]
    STRING,

    // Characters: Matches single-quoted characters, including escape sequences
    #[regex(r#"'([^'\\]|\\.)'"#)]
    CHAR,

    // Single dot
    #[token(".")]
    Dot,

    // Type tokens
    #[token("i8")]
    TYPEI8,
    #[token("i16")]
    TYPEI16,
    #[token("i32")]
    TYPEI32,
    #[token("i64")]
    TYPEI64,
    #[token("u8")]
    TYPEU8,
    #[token("u16")]
    TYPEU16,
    #[token("u32")]
    TYPEU32,
    #[token("u64")]
    TYPEU64,
    #[token("f32")]
    TYPEF32,
    #[token("f64")]
    TYPEF64,
    #[token("c32")]
    TYPEC32,
    #[token("c64")]
    TYPEC64,
    #[token("char")]
    TYPECHAR,
    #[token("string")]
    TYPESTRING,
    #[token("bool")]
    TYPEBOOL,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Integer,
    Double,
    Boolean,
    Plus,
    Minus,
    Not,
    Star,
    Divide,
    Xor,
    Percent,
    Or,
    And,
    Equal,
    Less,
    Greater,
    PlusPlus,
    MinusMinus,
    PlusEqual,
    MinusEqual,
    NotEqual,
    StarEqual,
    DivideEqual,
    XorEqual,
    PercentEqual,
    OrOr,
    AndAnd,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Dot,
    Identifier,
    Char,
    String,
    KMain,
    KVar,
    KIf,
    KWhile,
    KElse,
    KFor,
    KBreak,
    KFun,
    KReturn,
    KNullptr,
    OpenParenthesis,
    OpenSqParenthesis,
    OpenCurParenthesis,
    CloseParenthesis,
    CloseSqParenthesis,
    CloseCurParenthesis,
    Comma,
    Colon,
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeC32,
    TypeC64,
    TypeChar,
    TypeString,
    TypeBool,
    Comment,
    Unknown,
    Eoft,
}

#[allow(dead_code)]
impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::KMain
                | TokenType::KVar
                | TokenType::KIf
                | TokenType::KWhile
                | TokenType::KElse
                | TokenType::KFor
                | TokenType::KBreak
                | TokenType::KFun
                | TokenType::KReturn
        )
    }

    pub fn map_keword_to_token_type(keyword: String) -> TokenType {
        match keyword.as_str() {
            "main" => TokenType::KMain,
            "var" => TokenType::KVar,
            "if" => TokenType::KIf,
            "while" => TokenType::KWhile,
            "else" => TokenType::KElse,
            "for" => TokenType::KFor,
            "break" => TokenType::KBreak,
            "fun" => TokenType::KFun,
            "return" => TokenType::KReturn,
            _ => TokenType::Identifier,
        }
    }

    pub fn trim_start_and_end(raw: String) -> String {
        raw[1..raw.len() - 1].to_string()
    }

    pub fn map_raw_token_type(raw_token: RawToken, raw_val: String) -> (TokenType, &'static str) {
        match raw_token {
            RawToken::IdentifierAscii => (
                TokenType::map_keword_to_token_type(raw_val.clone()),
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::IdentifierUnicode => {
                (TokenType::Identifier, Box::leak(raw_val.into_boxed_str()))
            }
            RawToken::Binary => (TokenType::Integer, Box::leak(raw_val.into_boxed_str())),
            RawToken::Hexadecimal => (TokenType::Integer, Box::leak(raw_val.into_boxed_str())),
            RawToken::Octal => (TokenType::Integer, Box::leak(raw_val.into_boxed_str())),
            RawToken::Whitespace => (TokenType::Unknown, Box::leak(raw_val.into_boxed_str())),
            RawToken::SingleLineComment => {
                (TokenType::Comment, Box::leak(raw_val.into_boxed_str()))
            }
            RawToken::MultiLineComment => (TokenType::Comment, Box::leak(raw_val.into_boxed_str())),
            RawToken::Plus => (TokenType::Plus, Box::leak(raw_val.into_boxed_str())),
            RawToken::Minus => (TokenType::Minus, Box::leak(raw_val.into_boxed_str())),
            RawToken::Star => (TokenType::Star, Box::leak(raw_val.into_boxed_str())),
            RawToken::Slash => (TokenType::Divide, Box::leak(raw_val.into_boxed_str())),
            RawToken::LESS => (TokenType::Less, Box::leak(raw_val.into_boxed_str())),
            RawToken::GREATER => (TokenType::Greater, Box::leak(raw_val.into_boxed_str())),
            RawToken::NOT => (TokenType::Not, Box::leak(raw_val.into_boxed_str())),
            RawToken::XOR => (TokenType::Xor, Box::leak(raw_val.into_boxed_str())),
            RawToken::PERCENT => (TokenType::Percent, Box::leak(raw_val.into_boxed_str())),
            RawToken::OR => (TokenType::Or, Box::leak(raw_val.into_boxed_str())),
            RawToken::AND => (TokenType::And, Box::leak(raw_val.into_boxed_str())),
            RawToken::Equal => (TokenType::Equal, Box::leak(raw_val.into_boxed_str())),
            RawToken::Colon => (TokenType::Colon, Box::leak(raw_val.into_boxed_str())),
            RawToken::Comma => (TokenType::Comma, Box::leak(raw_val.into_boxed_str())),
            RawToken::PlusPlus => (TokenType::PlusPlus, Box::leak(raw_val.into_boxed_str())),
            RawToken::MinusMinus => (TokenType::MinusMinus, Box::leak(raw_val.into_boxed_str())),
            RawToken::PlusEqual => (TokenType::PlusEqual, Box::leak(raw_val.into_boxed_str())),
            RawToken::MinusEqual => (TokenType::MinusEqual, Box::leak(raw_val.into_boxed_str())),
            RawToken::LessEqual => (TokenType::LessEqual, Box::leak(raw_val.into_boxed_str())),
            RawToken::Greaterequal => {
                (TokenType::GreaterEqual, Box::leak(raw_val.into_boxed_str()))
            }
            RawToken::NotEqual => (TokenType::NotEqual, Box::leak(raw_val.into_boxed_str())),
            RawToken::XorEqual => (TokenType::XorEqual, Box::leak(raw_val.into_boxed_str())),
            RawToken::PercentEqual => {
                (TokenType::PercentEqual, Box::leak(raw_val.into_boxed_str()))
            }
            RawToken::OrOr => (TokenType::OrOr, Box::leak(raw_val.into_boxed_str())),
            RawToken::AndAnd => (TokenType::AndAnd, Box::leak(raw_val.into_boxed_str())),
            RawToken::OpenParentesis => (
                TokenType::OpenParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::CloseParentesis => (
                TokenType::CloseParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::OpenSQParentesis => (
                TokenType::OpenSqParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::CloseSQParentesis => (
                TokenType::CloseSqParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::OpenCurParentesis => (
                TokenType::OpenCurParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::CloseCurParentesis => (
                TokenType::CloseCurParenthesis,
                Box::leak(raw_val.into_boxed_str()),
            ),
            RawToken::BOOLEAN => (TokenType::Boolean, Box::leak(raw_val.into_boxed_str())),
            RawToken::STRING => (
                TokenType::String,
                Box::leak(
                    raw_val
                        .trim_start_matches('\"')
                        .trim_end_matches('\"')
                        .to_string()
                        .into_boxed_str(),
                ),
            ),
            RawToken::CHAR => (
                TokenType::Char,
                Box::leak(
                    raw_val
                        .trim_start_matches('\'')
                        .trim_end_matches('\'')
                        .to_string()
                        .into_boxed_str(),
                ),
            ),
            RawToken::Dot => (TokenType::Dot, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEI8 => (TokenType::TypeI8, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEI16 => (TokenType::TypeI16, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEI32 => (TokenType::TypeI32, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEI64 => (TokenType::TypeI64, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEU8 => (TokenType::TypeU8, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEU16 => (TokenType::TypeU16, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEU32 => (TokenType::TypeU32, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEU64 => (TokenType::TypeU64, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEF32 => (TokenType::TypeF32, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEF64 => (TokenType::TypeF64, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEC32 => (TokenType::TypeC32, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPEC64 => (TokenType::TypeC64, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPECHAR => (TokenType::TypeChar, Box::leak(raw_val.into_boxed_str())),
            RawToken::TYPESTRING => (TokenType::TypeString, Box::leak(raw_val.into_boxed_str())),
            RawToken::Number => {
                if raw_val.as_str().contains('.') {
                    (TokenType::Double, Box::leak(raw_val.into_boxed_str()))
                } else {
                    (TokenType::Integer, Box::leak(raw_val.into_boxed_str()))
                }
            }
            RawToken::TYPEBOOL => (TokenType::TypeBool, Box::leak(raw_val.into_boxed_str())),
            //_ => (TokenType::Unknown,raw_val),
        }
    }

    pub fn compact_to_string(&self) -> &'static str {
        match self {
            TokenType::Integer => "INT",
            TokenType::Double => "DBL",
            TokenType::Boolean => "BOOL",
            TokenType::Plus => "PLUS_OP",
            TokenType::Minus => "MINUS_OP",
            TokenType::Equal => "EQUAL_OP",
            TokenType::Dot => "DOT_OP",
            TokenType::Star => "STAR_OP",
            TokenType::Divide => "DIVIDE_OP",
            TokenType::Xor => "XOR_OP",
            TokenType::Percent => "PERCENT_OP",
            TokenType::Or => "OR_OP",
            TokenType::And => "AND_OP",
            TokenType::Less => "LESS_OP",
            TokenType::Greater => "GREATER_OP",
            TokenType::PlusPlus => "PLUSPLUS_OP",
            TokenType::MinusMinus => "MINUSMINUS_OP",
            TokenType::PlusEqual => "PLUSEQUAL_OP",
            TokenType::MinusEqual => "MINUSEQUAL_OP",
            TokenType::NotEqual => "NOTEQUAL_OP",
            TokenType::StarEqual => "STAREQUAL_OP",
            TokenType::DivideEqual => "DIVIDEEQUAL_OP",
            TokenType::XorEqual => "XOREQUAL_OP",
            TokenType::PercentEqual => "PERCENTEQUAL_OP",
            TokenType::OrOr => "OROR_OP",
            TokenType::AndAnd => "ANDAND_OP",
            TokenType::EqualEqual => "EQUALEQUAL_OP",
            TokenType::LessEqual => "LESSEQUAL_OP",
            TokenType::GreaterEqual => "GREATEREQUAL_OP",
            TokenType::Identifier => "IDENT",
            TokenType::Char => "CH",
            TokenType::String => "STR",
            TokenType::Eoft => "EOF",
            TokenType::KMain => "K_MAIN",
            TokenType::KVar => "K_VAR",
            TokenType::KIf => "K_IF",
            TokenType::KWhile => "K_WHILE",
            TokenType::KElse => "K_ELSE",
            TokenType::KFor => "K_FOR",
            TokenType::KBreak => "BREAK",
            TokenType::KFun => "K_FUN",
            TokenType::KReturn => "K_RETURN",
            TokenType::KNullptr => "K_NULLPTR",
            TokenType::OpenParenthesis => "OPEN_PAR",
            TokenType::OpenSqParenthesis => "OPEN_SQ_PAR",
            TokenType::OpenCurParenthesis => "OPEN_CUR_PAR",
            TokenType::CloseParenthesis => "CLOSE_PAR",
            TokenType::CloseSqParenthesis => "CLOSE_SQ_PAR",
            TokenType::CloseCurParenthesis => "CLOSE_CUR_PAR",
            TokenType::Not => "NOT_OP",
            TokenType::Comma => "COMMA",
            TokenType::Colon => "COLON",
            TokenType::TypeI8 => "I8",
            TokenType::TypeI16 => "I16",
            TokenType::TypeI32 => "I32",
            TokenType::TypeI64 => "I64",
            TokenType::TypeU8 => "U8",
            TokenType::TypeU16 => "U16",
            TokenType::TypeU32 => "U32",
            TokenType::TypeU64 => "U64",
            TokenType::TypeF32 => "F32",
            TokenType::TypeF64 => "F64",
            TokenType::TypeC32 => "C32",
            TokenType::TypeC64 => "C64",
            TokenType::TypeChar => "CHAR",
            TokenType::TypeString => "STRING",
            TokenType::TypeBool => "BOOL",
            TokenType::Comment => "COMMENT",
            // Add more cases as needed
            TokenType::Unknown => "UNKNOWN",
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.compact_to_string())
        } else {
            // Usa il formato standard.
            write!(f, "{:?}", self)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_keyword() {
        assert!(TokenType::KMain.is_keyword());
        assert!(TokenType::KVar.is_keyword());
        assert!(TokenType::KIf.is_keyword());
        assert!(TokenType::KWhile.is_keyword());
        assert!(TokenType::KElse.is_keyword());
        assert!(TokenType::KFor.is_keyword());
        assert!(TokenType::KBreak.is_keyword());
        assert!(TokenType::KFun.is_keyword());
        assert!(TokenType::KReturn.is_keyword());
        assert!(!TokenType::Identifier.is_keyword());
    }

    #[test]
    fn test_map_keyword_to_token_type() {
        assert_eq!(TokenType::map_keword_to_token_type("main".to_string()), TokenType::KMain);
        assert_eq!(TokenType::map_keword_to_token_type("var".to_string()), TokenType::KVar);
        assert_eq!(TokenType::map_keword_to_token_type("if".to_string()), TokenType::KIf);
        assert_eq!(TokenType::map_keword_to_token_type("while".to_string()), TokenType::KWhile);
        assert_eq!(TokenType::map_keword_to_token_type("else".to_string()), TokenType::KElse);
        assert_eq!(TokenType::map_keword_to_token_type("for".to_string()), TokenType::KFor);
        assert_eq!(TokenType::map_keword_to_token_type("break".to_string()), TokenType::KBreak);
        assert_eq!(TokenType::map_keword_to_token_type("fun".to_string()), TokenType::KFun);
        assert_eq!(TokenType::map_keword_to_token_type("return".to_string()), TokenType::KReturn);
        assert_eq!(TokenType::map_keword_to_token_type("unknown".to_string()), TokenType::Identifier);
    }

    #[test]
    fn test_trim_start_and_end() {
        assert_eq!(TokenType::trim_start_and_end("\"hello\"".to_string()), "hello");
        assert_eq!(TokenType::trim_start_and_end("'c'".to_string()), "c");
    }

    #[test]
    fn test_map_raw_token_type() {
        assert_eq!(TokenType::map_raw_token_type(RawToken::IdentifierAscii, "main".to_string()), (TokenType::KMain, "main"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::IdentifierUnicode, "变量".to_string()), (TokenType::Identifier, "变量"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Binary, "##1010".to_string()), (TokenType::Integer, "##1010"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Hexadecimal, "#ff".to_string()), (TokenType::Integer, "#ff"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Octal, "#o23".to_string()), (TokenType::Integer, "#o23"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Whitespace, " ".to_string()), (TokenType::Unknown, " "));
        assert_eq!(TokenType::map_raw_token_type(RawToken::SingleLineComment, "// comment".to_string()), (TokenType::Comment, "// comment"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::MultiLineComment, "/* comment */".to_string()), (TokenType::Comment, "/* comment */"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Plus, "+".to_string()), (TokenType::Plus, "+"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Minus, "-".to_string()), (TokenType::Minus, "-"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Star, "*".to_string()), (TokenType::Star, "*"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Slash, "/".to_string()), (TokenType::Divide, "/"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::LESS, "<".to_string()), (TokenType::Less, "<"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::GREATER, ">".to_string()), (TokenType::Greater, ">"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::NOT, "!".to_string()), (TokenType::Not, "!"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::XOR, "^".to_string()), (TokenType::Xor, "^"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::PERCENT, "%".to_string()), (TokenType::Percent, "%"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::OR, "|".to_string()), (TokenType::Or, "|"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::AND, "&".to_string()), (TokenType::And, "&"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Equal, "=".to_string()), (TokenType::Equal, "="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Colon, ":".to_string()), (TokenType::Colon, ":"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Comma, ",".to_string()), (TokenType::Comma, ","));
        assert_eq!(TokenType::map_raw_token_type(RawToken::PlusPlus, "++".to_string()), (TokenType::PlusPlus, "++"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::MinusMinus, "--".to_string()), (TokenType::MinusMinus, "--"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::PlusEqual, "+=".to_string()), (TokenType::PlusEqual, "+="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::MinusEqual, "-=".to_string()), (TokenType::MinusEqual, "-="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::LessEqual, "<=".to_string()), (TokenType::LessEqual, "<="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Greaterequal, ">=".to_string()), (TokenType::GreaterEqual, ">="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::NotEqual, "!=".to_string()), (TokenType::NotEqual, "!="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::XorEqual, "^=".to_string()), (TokenType::XorEqual, "^="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::PercentEqual, "%=".to_string()), (TokenType::PercentEqual, "%="));
        assert_eq!(TokenType::map_raw_token_type(RawToken::OrOr, "||".to_string()), (TokenType::OrOr, "||"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::AndAnd, "&&".to_string()), (TokenType::AndAnd, "&&"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::OpenParentesis, "(".to_string()), (TokenType::OpenParenthesis, "("));
        assert_eq!(TokenType::map_raw_token_type(RawToken::CloseParentesis, ")".to_string()), (TokenType::CloseParenthesis, ")"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::OpenSQParentesis, "[".to_string()), (TokenType::OpenSqParenthesis, "["));
        assert_eq!(TokenType::map_raw_token_type(RawToken::CloseSQParentesis, "]".to_string()), (TokenType::CloseSqParenthesis, "]"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::OpenCurParentesis, "{".to_string()), (TokenType::OpenCurParenthesis, "{"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::CloseCurParentesis, "}".to_string()), (TokenType::CloseCurParenthesis, "}"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::BOOLEAN, "true".to_string()), (TokenType::Boolean, "true"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::STRING, "\"hello\"".to_string()), (TokenType::String, "hello"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::CHAR, "'c'".to_string()), (TokenType::Char, "c"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Dot, ".".to_string()), (TokenType::Dot, "."));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEI8, "i8".to_string()), (TokenType::TypeI8, "i8"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEI16, "i16".to_string()), (TokenType::TypeI16, "i16"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEI32, "i32".to_string()), (TokenType::TypeI32, "i32"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEI64, "i64".to_string()), (TokenType::TypeI64, "i64"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEU8, "u8".to_string()), (TokenType::TypeU8, "u8"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEU16, "u16".to_string()), (TokenType::TypeU16, "u16"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEU32, "u32".to_string()), (TokenType::TypeU32, "u32"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEU64, "u64".to_string()), (TokenType::TypeU64, "u64"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEF32, "f32".to_string()), (TokenType::TypeF32, "f32"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEF64, "f64".to_string()), (TokenType::TypeF64, "f64"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEC32, "c32".to_string()), (TokenType::TypeC32, "c32"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEC64, "c64".to_string()), (TokenType::TypeC64, "c64"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPECHAR, "char".to_string()), (TokenType::TypeChar, "char"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPESTRING, "string".to_string()), (TokenType::TypeString, "string"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::TYPEBOOL, "bool".to_string()), (TokenType::TypeBool, "bool"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Number, "123".to_string()), (TokenType::Integer, "123"));
        assert_eq!(TokenType::map_raw_token_type(RawToken::Number, "123.45".to_string()), (TokenType::Double, "123.45"));
    }

    #[test]
    fn test_compact_to_string() {
        assert_eq!(TokenType::Integer.compact_to_string(), "INT");
        assert_eq!(TokenType::Double.compact_to_string(), "DBL");
        assert_eq!(TokenType::Boolean.compact_to_string(), "BOOL");
        assert_eq!(TokenType::Plus.compact_to_string(), "PLUS_OP");
        assert_eq!(TokenType::Minus.compact_to_string(), "MINUS_OP");
        assert_eq!(TokenType::Equal.compact_to_string(), "EQUAL_OP");
        assert_eq!(TokenType::Dot.compact_to_string(), "DOT_OP");
        assert_eq!(TokenType::Star.compact_to_string(), "STAR_OP");
        assert_eq!(TokenType::Divide.compact_to_string(), "DIVIDE_OP");
        assert_eq!(TokenType::Xor.compact_to_string(), "XOR_OP");
        assert_eq!(TokenType::Percent.compact_to_string(), "PERCENT_OP");
        assert_eq!(TokenType::Or.compact_to_string(), "OR_OP");
        assert_eq!(TokenType::And.compact_to_string(), "AND_OP");
        assert_eq!(TokenType::Less.compact_to_string(), "LESS_OP");
        assert_eq!(TokenType::Greater.compact_to_string(), "GREATER_OP");
        assert_eq!(TokenType::PlusPlus.compact_to_string(), "PLUSPLUS_OP");
        assert_eq!(TokenType::MinusMinus.compact_to_string(), "MINUSMINUS_OP");
        assert_eq!(TokenType::PlusEqual.compact_to_string(), "PLUSEQUAL_OP");
        assert_eq!(TokenType::MinusEqual.compact_to_string(), "MINUSEQUAL_OP");
        assert_eq!(TokenType::NotEqual.compact_to_string(), "NOTEQUAL_OP");
        assert_eq!(TokenType::StarEqual.compact_to_string(), "STAREQUAL_OP");
        assert_eq!(TokenType::DivideEqual.compact_to_string(), "DIVIDEEQUAL_OP");
        assert_eq!(TokenType::XorEqual.compact_to_string(), "XOREQUAL_OP");
        assert_eq!(TokenType::PercentEqual.compact_to_string(), "PERCENTEQUAL_OP");
        assert_eq!(TokenType::OrOr.compact_to_string(), "OROR_OP");
        assert_eq!(TokenType::AndAnd.compact_to_string(), "ANDAND_OP");
        assert_eq!(TokenType::EqualEqual.compact_to_string(), "EQUALEQUAL_OP");
        assert_eq!(TokenType::LessEqual.compact_to_string(), "LESSEQUAL_OP");
        assert_eq!(TokenType::GreaterEqual.compact_to_string(), "GREATEREQUAL_OP");
        assert_eq!(TokenType::Identifier.compact_to_string(), "IDENT");
        assert_eq!(TokenType::Char.compact_to_string(), "CH");
        assert_eq!(TokenType::String.compact_to_string(), "STR");
        assert_eq!(TokenType::Eoft.compact_to_string(), "EOF");
        assert_eq!(TokenType::KMain.compact_to_string(), "K_MAIN");
        assert_eq!(TokenType::KVar.compact_to_string(), "K_VAR");
        assert_eq!(TokenType::KIf.compact_to_string(), "K_IF");
        assert_eq!(TokenType::KWhile.compact_to_string(), "K_WHILE");
        assert_eq!(TokenType::KElse.compact_to_string(), "K_ELSE");
        assert_eq!(TokenType::KFor.compact_to_string(), "K_FOR");
        assert_eq!(TokenType::KBreak.compact_to_string(), "BREAK");
        assert_eq!(TokenType::KFun.compact_to_string(), "K_FUN");
        assert_eq!(TokenType::KReturn.compact_to_string(), "K_RETURN");
        assert_eq!(TokenType::KNullptr.compact_to_string(), "K_NULLPTR");
        assert_eq!(TokenType::OpenParenthesis.compact_to_string(), "OPEN_PAR");
        assert_eq!(TokenType::OpenSqParenthesis.compact_to_string(), "OPEN_SQ_PAR");
        assert_eq!(TokenType::OpenCurParenthesis.compact_to_string(), "OPEN_CUR_PAR");
        assert_eq!(TokenType::CloseParenthesis.compact_to_string(), "CLOSE_PAR");
        assert_eq!(TokenType::CloseSqParenthesis.compact_to_string(), "CLOSE_SQ_PAR");
        assert_eq!(TokenType::CloseCurParenthesis.compact_to_string(), "CLOSE_CUR_PAR");
        assert_eq!(TokenType::Not.compact_to_string(), "NOT_OP");
        assert_eq!(TokenType::Comma.compact_to_string(), "COMMA");
        assert_eq!(TokenType::Colon.compact_to_string(), "COLON");
        assert_eq!(TokenType::TypeI8.compact_to_string(), "I8");
        assert_eq!(TokenType::TypeI16.compact_to_string(), "I16");
        assert_eq!(TokenType::TypeI32.compact_to_string(), "I32");
        assert_eq!(TokenType::TypeI64.compact_to_string(), "I64");
        assert_eq!(TokenType::TypeU8.compact_to_string(), "U8");
        assert_eq!(TokenType::TypeU16.compact_to_string(), "U16");
        assert_eq!(TokenType::TypeU32.compact_to_string(), "U32");
        assert_eq!(TokenType::TypeU64.compact_to_string(), "U64");
        assert_eq!(TokenType::TypeF32.compact_to_string(), "F32");
        assert_eq!(TokenType::TypeF64.compact_to_string(), "F64");
        assert_eq!(TokenType::TypeC32.compact_to_string(), "C32");
        assert_eq!(TokenType::TypeC64.compact_to_string(), "C64");
        assert_eq!(TokenType::TypeChar.compact_to_string(), "CHAR");
        assert_eq!(TokenType::TypeString.compact_to_string(), "STRING");
        assert_eq!(TokenType::TypeBool.compact_to_string(), "BOOL");
        assert_eq!(TokenType::Comment.compact_to_string(), "COMMENT");
        assert_eq!(TokenType::Unknown.compact_to_string(), "UNKNOWN");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TokenType::Integer), "Integer");
        assert_eq!(format!("{:#}", TokenType::Integer), "INT");
    }
}
