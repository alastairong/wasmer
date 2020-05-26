use crate::{
    error::RuntimeError,
    new,
    types::FuncDescriptor,
    types::{Type, Value},
};

pub struct Func {
    new_function: new::wasmer::Function,
    signature: FuncDescriptor,
}

impl Func {
    pub fn new<F, Args, Rets, Env>(func: F) -> Self
    where
        F: new::wasm_common::HostFunction<Args, Rets, new::wasm_common::WithoutEnv, Env>,
        Args: new::wasm_common::WasmTypeList,
        Rets: new::wasm_common::WasmTypeList,
        Env: Sized,
    {
        let store = Default::default();
        let new_function = new::wasmer::Function::new::<F, Args, Rets, Env>(&store, func);
        let signature = new_function.ty();

        Self {
            new_function,
            signature,
        }
    }

    pub fn new_env<F, Args, Rets, Env>(env: &mut Env, func: F) -> Self
    where
        F: new::wasm_common::HostFunction<Args, Rets, new::wasm_common::WithEnv, Env>,
        Args: new::wasm_common::WasmTypeList,
        Rets: new::wasm_common::WasmTypeList,
        Env: Sized,
    {
        let store = Default::default();
        let new_function = new::wasmer::Function::new_env::<F, Args, Rets, Env>(&store, env, func);
        let signature = new_function.ty();

        Self {
            new_function,
            signature,
        }
    }

    pub fn new_dynamic<F>(ty: &FuncDescriptor, func: F) -> Self
    where
        F: Fn(&[Value]) -> Result<Vec<Value>, RuntimeError> + 'static,
    {
        let store = Default::default();
        let new_function = new::wasmer::Function::new_dynamic(&store, ty, func);
        let signature = new_function.ty();

        Self {
            new_function,
            signature,
        }
    }

    pub fn new_dynamic_env<F, Env>(ty: &FuncDescriptor, env: &mut Env, func: F) -> Self
    where
        F: Fn(&mut Env, &[Value]) -> Result<Vec<Value>, RuntimeError> + 'static,
        Env: Sized,
    {
        let store = Default::default();
        let new_function = new::wasmer::Function::new_dynamic_env::<F, Env>(&store, ty, env, func);
        let signature = new_function.ty();

        Self {
            new_function,
            signature,
        }
    }

    pub fn signature(&self) -> &FuncDescriptor {
        &self.signature
    }

    pub fn params(&self) -> &[Type] {
        self.signature().params()
    }

    pub fn returns(&self) -> &[Type] {
        self.signature().results()
    }

    pub fn call(&self, params: &[Value]) -> Result<Box<[Value]>, RuntimeError> {
        self.new_function.call(params)
    }
}
