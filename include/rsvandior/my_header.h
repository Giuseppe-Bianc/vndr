#pragma once

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


enum class TokenType {
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
};

struct CodeSourceLocation {
    const char *file_name;
    size_t line;
    size_t column;

    CodeSourceLocation(const char *const& file_name,
                       size_t const& line,
                       size_t const& column)
      : file_name(file_name),
        line(line),
        column(column)
    {}

    bool operator==(const CodeSourceLocation& other) const {
        return file_name == other.file_name &&
               line == other.line &&
               column == other.column;
    }
};

struct Token {
    TokenType token_type;
    const char *value;
    CodeSourceLocation source_location;

    Token(TokenType const& token_type,
          const char *const& value,
          CodeSourceLocation const& source_location)
      : token_type(token_type),
        value(value),
        source_location(source_location)
    {}

    bool operator==(const Token& other) const {
        return token_type == other.token_type &&
               value == other.value &&
               source_location == other.source_location;
    }
};

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct {
        Token* tokens;
        size_t size;
        size_t capacity;
    } TokenDynamicArray;


int32_t add_numbers(int32_t a, int32_t b);

int32_t subtract_numbers(int32_t a, int32_t b);

TokenDynamicArray tokenize_w(const char *file_name, const char *input);
#ifdef __cplusplus
}
#endif