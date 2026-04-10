#!/usr/bin/env bash

set -euo pipefail

clang-format --dry-run --Werror main.cpp

mkdir -p build && cd build
cmake ..
make

valgrind --error-exitcode=1 \
  --tool=memcheck --leak-check=full \
  ./ffi-utilities-demo-c-executable
