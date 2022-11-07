use std::collections::HashMap;
use llvm_sys::prelude::LLVMValueRef;

use iron_llvm::core;
use iron_llvm::core::types::{RealTypeCtor, RealTypeRef};
use iron_llvm::{LLVMRef, LLVMRefCtor};

pub struct Context {
    context: core::Context,
    builder: core::builder,
    variables: HashMap<String, LLVMValueRef>,
    ty: RealTypeRef
}

impl Context {
    pub fn new() -> Context {

        let context = core::Context::get_global();
        let builder = core::Builder::new();
        let variables = HashMap::new();
        let ty = RealTypeRef::get_double();

        Context {
            context,
            builder,
            variables,
            ty
        }
    }
}