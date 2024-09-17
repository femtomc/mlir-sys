#!/bin/sh

set -e

llvm_version=19

brew update
brew install llvm@$llvm_version

echo MLIR_SYS_${llvm_version}0_PREFIX=$(brew --prefix llvm@$llvm_version) >>$GITHUB_ENV
