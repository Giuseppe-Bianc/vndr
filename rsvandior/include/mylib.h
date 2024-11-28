#include <cstdarg>
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

struct String;

template<typename T = void>
struct Vec;

struct CodeSourceLocation {
  String file_name;
  uintptr_t line;
  uintptr_t column;
};

struct Token {
  TokenType token_type;
  String value;
  CodeSourceLocation source_location;
};

struct TokenList {
  String file_name;
  Vec<Token> tokens;
};

extern "C" {

int32_t add_numbers(int32_t a, int32_t b);

int32_t subtract_numbers(int32_t a, int32_t b);

TokenList tokenize(const str *file_name, const str *input);

}  // extern "C"
