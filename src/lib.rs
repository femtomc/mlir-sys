/*

   This file is part of mlir-sys. License is MIT.

*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(unused_must_use)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod mlir_tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_context() {
        unsafe {
            let ctx = mlirContextCreate();
            assert!(mlirContextEqual(ctx, ctx));
            mlirContextDestroy(ctx);
        }
    }

    #[test]
    fn test_stringref() {
        unsafe {
            let c_to_print = CString::new("Hello, world!").expect("CString::new failed");
            let r = mlirStringRefCreateFromCString(c_to_print.as_ptr());
        }
    }

    #[test]
    fn test_location() {
        unsafe {
            let ctx = mlirContextCreate();
            mlirRegisterAllDialects(ctx);
            let loc = mlirLocationUnknownGet(ctx);
            let c_to_print = CString::new("newmod").expect("CString::new failed");
            let r = mlirStringRefCreateFromCString(c_to_print.as_ptr());
            let opstate = mlirOperationStateGet(r, loc);
        }
    }
}
