#!/bin/sh

set -e

llvm_version=15

brew update
brew install llvm@$llvm_version

llvm_prefix=$(brew --prefix)/opt/llvm@$llvm_version

echo MLIR_SYS_150_PREFIX=$llvm_prefix >>$GITHUB_ENV
echo PATH=$llvm_prefix/bin:$PATH >>$GITHUB_ENV
