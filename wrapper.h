#include <mlir-c/AffineExpr.h>
#include <mlir-c/AffineMap.h>
#include <mlir-c/BuiltinAttributes.h>
#include <mlir-c/BuiltinTypes.h>
#include <mlir-c/Conversion.h>
#include <mlir-c/Debug.h>
#include <mlir-c/Diagnostics.h>
#include <mlir-c/Dialect/Async.h>
#include <mlir-c/Dialect/GPU.h>
#include <mlir-c/Dialect/Linalg.h>
#include <mlir-c/Dialect/LLVM.h>
#include <mlir-c/Dialect/PDL.h>
#include <mlir-c/Dialect/Quant.h>
#include <mlir-c/Dialect/SCF.h>
#include <mlir-c/Dialect/Shape.h>
#include <mlir-c/Dialect/SparseTensor.h>
#include <mlir-c/Dialect/Standard.h>
#include <mlir-c/Dialect/Tensor.h>
#include <mlir-c/ExecutionEngine.h>
#include <mlir-c/IntegerSet.h>
// TODO This should be fixed on LLVM >14.0.6.
// #include <mlir-c/Interfaces.h>
#include <mlir-c/IR.h>
#include <mlir-c/Pass.h>
#include <mlir-c/Registration.h>
#include <mlir-c/Support.h>
#include <mlir-c/Transforms.h>
