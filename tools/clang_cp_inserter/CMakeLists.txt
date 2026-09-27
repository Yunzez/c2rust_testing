cmake_minimum_required(VERSION 3.16)
project(clang_cp_inserter LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(LLVM REQUIRED CONFIG)
find_package(Clang REQUIRED CONFIG)

include_directories(${LLVM_INCLUDE_DIRS} ${CLANG_INCLUDE_DIRS})
add_definitions(${LLVM_DEFINITIONS})

add_executable(clang_cp_inserter main.cpp)

llvm_map_components_to_libnames(LLVM_LIBS support)

set(CLANG_LIBS
  clangTooling
  clangBasic
  clangASTMatchers
  clangAST
  clangFrontend
  clangRewrite
  clangLex
)

target_link_libraries(clang_cp_inserter PRIVATE ${LLVM_LIBS} ${CLANG_LIBS})
