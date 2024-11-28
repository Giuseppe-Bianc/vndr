/*
* Created by gbian on 28/11/2024.
* Copyright (c) 2024 All rights reserved.
*/

#pragma once

#include "my_header.h"
#include <string>
#include <string_view>
#include <vector>

namespace vnd {
       inline std::vector<::Token> wrap_tokenize(const std::string_view &input, std::string_view fileName) {
  const auto array = tokenize_w(input.data(), fileName.data());
         if (!array.tokens || array.size >array.capacity) {
           throw std::runtime_error("Invalid DynamicArray state");
         }

         std::vector<::Token> vec(array.tokens, array.tokens + array.size); // Transfer elements
         return vec;
       }
}