/*
 * Created by gbian on 28/11/2024.
 * Copyright (c) 2024 All rights reserved.
 */

#pragma once

#include "lexer/Token.hpp"
#include "rsvandior/my_header_wrapper.hpp"

inline vnd::TokenType tokenType_to_vnd_tokenType(::TokenType tokentype) {
  switch (tokentype) {
  case ::TokenType::Integer:
    return vnd::TokenType::INTEGER;
  case ::TokenType::Double:
    return vnd::TokenType::DOUBLE;
  case ::TokenType::Boolean:
    return vnd::TokenType::BOOLEAN;
  case ::TokenType::Plus:
    return vnd::TokenType::PLUS;
  case ::TokenType::Minus:
    return vnd::TokenType::MINUS;
  case ::TokenType::Not:
    return vnd::TokenType::NOT;
  case ::TokenType::Star:
    return vnd::TokenType::STAR;
  case ::TokenType::Divide:
    return vnd::TokenType::DIVIDE;
  case ::TokenType::Xor:
    return vnd::TokenType::XOR;
  case ::TokenType::Percent:
    return vnd::TokenType::PERCENT;
  case ::TokenType::Or:
    return vnd::TokenType::OR;
  case ::TokenType::And:
    return vnd::TokenType::AND;
  case ::TokenType::Equal:
    return vnd::TokenType::EQUAL;
  case ::TokenType::Less:
    return vnd::TokenType::LESS;
  case ::TokenType::Greater:
    return vnd::TokenType::GREATER;
  case ::TokenType::PlusPlus:
    return vnd::TokenType::PLUSPLUS;
  case ::TokenType::MinusMinus:
    return vnd::TokenType::MINUSMINUS;
  case ::TokenType::PlusEqual:
    return vnd::TokenType::PLUSEQUAL;
  case ::TokenType::NotEqual:
    return vnd::TokenType::NOTEQUAL;
  case ::TokenType::MinusEqual:
    return vnd::TokenType::MINUSEQUAL;
  case ::TokenType::DivideEqual:
    return vnd::TokenType::DIVIDEEQUAL;
  case ::TokenType::StarEqual:
    return vnd::TokenType::STAREQUAL;
  case ::TokenType::PercentEqual:
    return vnd::TokenType::PERCENTEQUAL;
  case ::TokenType::XorEqual:
    return vnd::TokenType::XOREQUAL;
  case ::TokenType::OrOr:
    return vnd::TokenType::OROR;
  case ::TokenType::AndAnd:
    return vnd::TokenType::ANDAND;
  case ::TokenType::EqualEqual:
    return vnd::TokenType::EQUALEQUAL;
  case ::TokenType::LessEqual:
    return vnd::TokenType::LESSEQUAL;
  case ::TokenType::GreaterEqual:
    return vnd::TokenType::GREATEREQUAL;
  case ::TokenType::Dot:
    return vnd::TokenType::DOT;
  case ::TokenType::Identifier:
    return vnd::TokenType::IDENTIFIER;
  case ::TokenType::Char:
    return vnd::TokenType::CHAR;
  case ::TokenType::String:
    return vnd::TokenType::STRING;
  case ::TokenType::KMain:
    return vnd::TokenType::K_MAIN;
  case ::TokenType::KVar:
    return vnd::TokenType::K_VAR;
  case ::TokenType::KIf:
    return vnd::TokenType::K_IF;
  case ::TokenType::KWhile:
    return vnd::TokenType::K_WHILE;
  case ::TokenType::KElse:
    return vnd::TokenType::K_ELSE;
  case ::TokenType::KFor:
    return vnd::TokenType::K_FOR;
  case ::TokenType::KBreak:
    return vnd::TokenType::K_BREAK;
  case ::TokenType::KFun:
    return vnd::TokenType::K_FUN;
  case ::TokenType::KReturn:
    return vnd::TokenType::K_RETURN;
  case ::TokenType::KNullptr:
    return vnd::TokenType::K_NULLPTR;
  case ::TokenType::OpenParenthesis:
    return vnd::TokenType::OPEN_PARENTESIS;
  case ::TokenType::OpenSqParenthesis:
    return vnd::TokenType::OPEN_SQ_PARENTESIS;
  case ::TokenType::OpenCurParenthesis:
    return vnd::TokenType::OPEN_CUR_PARENTESIS;
  case ::TokenType::CloseParenthesis:
    return vnd::TokenType::CLOSE_PARENTESIS;
  case ::TokenType::CloseSqParenthesis:
    return vnd::TokenType::CLOSE_SQ_PARENTESIS;
  case ::TokenType::CloseCurParenthesis:
    return vnd::TokenType::CLOSE_CUR_PARENTESIS;
  case ::TokenType::Comma:
    return vnd::TokenType::COMMA;
  case ::TokenType::Colon:
    return vnd::TokenType::COLON;
  case ::TokenType::TypeI8:
    return vnd::TokenType::TYPE_I8;
  case ::TokenType::TypeI16:
    return vnd::TokenType::TYPE_I16;
  case ::TokenType::TypeI32:
    return vnd::TokenType::TYPE_I32;
  case ::TokenType::TypeI64:
    return vnd::TokenType::TYPE_I64;
  case ::TokenType::TypeU8:
    return vnd::TokenType::TYPE_U8;
  case ::TokenType::TypeU16:
    return vnd::TokenType::TYPE_U16;
  case ::TokenType::TypeU32:
    return vnd::TokenType::TYPE_U32;
  case ::TokenType::TypeU64:
    return vnd::TokenType::TYPE_U64;
  case ::TokenType::TypeF32:
    return vnd::TokenType::TYPE_F32;
  case ::TokenType::TypeF64:
    return vnd::TokenType::TYPE_F64;
  case ::TokenType::TypeC32:
    return vnd::TokenType::TYPE_C32;
  case ::TokenType::TypeC64:
    return vnd::TokenType::TYPE_C64;
  case ::TokenType::TypeChar:
    return vnd::TokenType::TYPE_CHAR;
  case ::TokenType::TypeString:
    return vnd::TokenType::TYPE_STRING;
  case ::TokenType::TypeBool:
    return vnd::TokenType::TYPE_BOOL;
  case ::TokenType::Comment:
    return vnd::TokenType::COMMENT;
  case ::TokenType::Unknown:
    return vnd::TokenType::UNKNOWN;
  case ::TokenType::Eoft:
    return vnd::TokenType::EOFT;
  default:
    return vnd::TokenType::UNKNOWN;
  };
}

inline vnd::CodeSourceLocation to_vnd_codeSourceLocation(const ::CodeSourceLocation &source_location)
{
  return vnd::CodeSourceLocation(std::string_view{source_location.file_name}, source_location.line, source_location.column);
}

inline vnd::Token to_vnd_token(const ::Token &token)
{
  return vnd::Token(tokenType_to_vnd_tokenType(token.token_type),std::string_view{token.value}, to_vnd_codeSourceLocation(token.source_location));
}

inline std::vector<vnd::Token> to_vector_vnd_tokens(const std::vector<::Token> &tokens)
{
  std::vector<vnd::Token> vec;
  for (const auto &token : tokens) {
    vec.emplace_back(to_vnd_token(token));
  }
  return vec;
}
