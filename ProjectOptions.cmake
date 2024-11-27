include(cmake/SystemLink.cmake)
include(cmake/LibFuzzer.cmake)
include(CMakeDependentOption)
include(CheckCXXCompilerFlag)


macro(vndr_supports_sanitizers)
  if((CMAKE_CXX_COMPILER_ID MATCHES ".*Clang.*" OR CMAKE_CXX_COMPILER_ID MATCHES ".*GNU.*") AND NOT WIN32)
    set(SUPPORTS_UBSAN ON)
  else()
    set(SUPPORTS_UBSAN OFF)
  endif()

  if((CMAKE_CXX_COMPILER_ID MATCHES ".*Clang.*" OR CMAKE_CXX_COMPILER_ID MATCHES ".*GNU.*") AND WIN32)
    set(SUPPORTS_ASAN OFF)
  else()
    set(SUPPORTS_ASAN ON)
  endif()
endmacro()

macro(vndr_setup_options)
  option(vndr_ENABLE_HARDENING "Enable hardening" ON)
  option(vndr_ENABLE_COVERAGE "Enable coverage reporting" OFF)
  cmake_dependent_option(
    vndr_ENABLE_GLOBAL_HARDENING
    "Attempt to push hardening options to built dependencies"
    ON
    vndr_ENABLE_HARDENING
    OFF)

  vndr_supports_sanitizers()

  if(NOT PROJECT_IS_TOP_LEVEL OR vndr_PACKAGING_MAINTAINER_MODE)
    option(vndr_ENABLE_IPO "Enable IPO/LTO" OFF)
    option(vndr_WARNINGS_AS_ERRORS "Treat Warnings As Errors" OFF)
    option(vndr_ENABLE_USER_LINKER "Enable user-selected linker" OFF)
    option(vndr_ENABLE_SANITIZER_ADDRESS "Enable address sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_LEAK "Enable leak sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_UNDEFINED "Enable undefined sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_THREAD "Enable thread sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_MEMORY "Enable memory sanitizer" OFF)
    option(vndr_ENABLE_UNITY_BUILD "Enable unity builds" OFF)
    option(vndr_ENABLE_CLANG_TIDY "Enable clang-tidy" OFF)
    option(vndr_ENABLE_CPPCHECK "Enable cpp-check analysis" OFF)
    option(vndr_ENABLE_PCH "Enable precompiled headers" OFF)
    option(vndr_ENABLE_CACHE "Enable ccache" OFF)
  else()
    option(vndr_ENABLE_IPO "Enable IPO/LTO" ON)
    option(vndr_WARNINGS_AS_ERRORS "Treat Warnings As Errors" ON)
    option(vndr_ENABLE_USER_LINKER "Enable user-selected linker" OFF)
    option(vndr_ENABLE_SANITIZER_ADDRESS "Enable address sanitizer" ${SUPPORTS_ASAN})
    option(vndr_ENABLE_SANITIZER_LEAK "Enable leak sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_UNDEFINED "Enable undefined sanitizer" ${SUPPORTS_UBSAN})
    option(vndr_ENABLE_SANITIZER_THREAD "Enable thread sanitizer" OFF)
    option(vndr_ENABLE_SANITIZER_MEMORY "Enable memory sanitizer" OFF)
    option(vndr_ENABLE_UNITY_BUILD "Enable unity builds" OFF)
    option(vndr_ENABLE_CLANG_TIDY "Enable clang-tidy" ON)
    option(vndr_ENABLE_CPPCHECK "Enable cpp-check analysis" ON)
    option(vndr_ENABLE_PCH "Enable precompiled headers" OFF)
    option(vndr_ENABLE_CACHE "Enable ccache" ON)
  endif()

  if(NOT PROJECT_IS_TOP_LEVEL)
    mark_as_advanced(
      vndr_ENABLE_IPO
      vndr_WARNINGS_AS_ERRORS
      vndr_ENABLE_USER_LINKER
      vndr_ENABLE_SANITIZER_ADDRESS
      vndr_ENABLE_SANITIZER_LEAK
      vndr_ENABLE_SANITIZER_UNDEFINED
      vndr_ENABLE_SANITIZER_THREAD
      vndr_ENABLE_SANITIZER_MEMORY
      vndr_ENABLE_UNITY_BUILD
      vndr_ENABLE_CLANG_TIDY
      vndr_ENABLE_CPPCHECK
      vndr_ENABLE_COVERAGE
      vndr_ENABLE_PCH
      vndr_ENABLE_CACHE)
  endif()

  vndr_check_libfuzzer_support(LIBFUZZER_SUPPORTED)
  if(LIBFUZZER_SUPPORTED AND (vndr_ENABLE_SANITIZER_ADDRESS OR vndr_ENABLE_SANITIZER_THREAD OR vndr_ENABLE_SANITIZER_UNDEFINED))
    set(DEFAULT_FUZZER ON)
  else()
    set(DEFAULT_FUZZER OFF)
  endif()

  option(vndr_BUILD_FUZZ_TESTS "Enable fuzz testing executable" ${DEFAULT_FUZZER})

endmacro()

macro(vndr_global_options)
  if(vndr_ENABLE_IPO)
    include(cmake/InterproceduralOptimization.cmake)
    vndr_enable_ipo()
  endif()

  vndr_supports_sanitizers()

  if(vndr_ENABLE_HARDENING AND vndr_ENABLE_GLOBAL_HARDENING)
    include(cmake/Hardening.cmake)
    if(NOT SUPPORTS_UBSAN 
       OR vndr_ENABLE_SANITIZER_UNDEFINED
       OR vndr_ENABLE_SANITIZER_ADDRESS
       OR vndr_ENABLE_SANITIZER_THREAD
       OR vndr_ENABLE_SANITIZER_LEAK)
      set(ENABLE_UBSAN_MINIMAL_RUNTIME FALSE)
    else()
      set(ENABLE_UBSAN_MINIMAL_RUNTIME TRUE)
    endif()
    message("${vndr_ENABLE_HARDENING} ${ENABLE_UBSAN_MINIMAL_RUNTIME} ${vndr_ENABLE_SANITIZER_UNDEFINED}")
    vndr_enable_hardening(vndr_options ON ${ENABLE_UBSAN_MINIMAL_RUNTIME})
  endif()
endmacro()

macro(vndr_local_options)
  if(PROJECT_IS_TOP_LEVEL)
    include(cmake/StandardProjectSettings.cmake)
  endif()

  add_library(vndr_warnings INTERFACE)
  add_library(vndr_options INTERFACE)

  include(cmake/CompilerWarnings.cmake)
  vndr_set_project_warnings(
    vndr_warnings
    ${vndr_WARNINGS_AS_ERRORS}
    ""
    ""
    ""
    "")

  if(vndr_ENABLE_USER_LINKER)
    include(cmake/Linker.cmake)
    vndr_configure_linker(vndr_options)
  endif()

  include(cmake/Sanitizers.cmake)
  vndr_enable_sanitizers(
    vndr_options
    ${vndr_ENABLE_SANITIZER_ADDRESS}
    ${vndr_ENABLE_SANITIZER_LEAK}
    ${vndr_ENABLE_SANITIZER_UNDEFINED}
    ${vndr_ENABLE_SANITIZER_THREAD}
    ${vndr_ENABLE_SANITIZER_MEMORY})

  set_target_properties(vndr_options PROPERTIES UNITY_BUILD ${vndr_ENABLE_UNITY_BUILD})

  if(vndr_ENABLE_PCH)
    target_precompile_headers(
      vndr_options
      INTERFACE
      <vector>
      <string>
      <utility>)
  endif()

  if(vndr_ENABLE_CACHE)
    include(cmake/Cache.cmake)
    vndr_enable_cache()
  endif()

  include(cmake/StaticAnalyzers.cmake)
  if(vndr_ENABLE_CLANG_TIDY)
    vndr_enable_clang_tidy(vndr_options ${vndr_WARNINGS_AS_ERRORS})
  endif()

  if(vndr_ENABLE_CPPCHECK)
    vndr_enable_cppcheck(${vndr_WARNINGS_AS_ERRORS} "" # override cppcheck options
    )
  endif()

  if(vndr_ENABLE_COVERAGE)
    include(cmake/Tests.cmake)
    vndr_enable_coverage(vndr_options)
  endif()

  if(vndr_WARNINGS_AS_ERRORS)
    check_cxx_compiler_flag("-Wl,--fatal-warnings" LINKER_FATAL_WARNINGS)
    if(LINKER_FATAL_WARNINGS)
      # This is not working consistently, so disabling for now
      # target_link_options(vndr_options INTERFACE -Wl,--fatal-warnings)
    endif()
  endif()

  if(vndr_ENABLE_HARDENING AND NOT vndr_ENABLE_GLOBAL_HARDENING)
    include(cmake/Hardening.cmake)
    if(NOT SUPPORTS_UBSAN 
       OR vndr_ENABLE_SANITIZER_UNDEFINED
       OR vndr_ENABLE_SANITIZER_ADDRESS
       OR vndr_ENABLE_SANITIZER_THREAD
       OR vndr_ENABLE_SANITIZER_LEAK)
      set(ENABLE_UBSAN_MINIMAL_RUNTIME FALSE)
    else()
      set(ENABLE_UBSAN_MINIMAL_RUNTIME TRUE)
    endif()
    vndr_enable_hardening(vndr_options OFF ${ENABLE_UBSAN_MINIMAL_RUNTIME})
  endif()

endmacro()
