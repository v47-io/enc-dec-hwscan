/*
 * Copyright (C) 2024 Media Server 47 Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::zeroed;
use std::panic::{catch_unwind, panic_any, UnwindSafe};
use std::sync::Mutex;

use dylib_types::*;

use crate::device::CudaDevice;
use crate::dylib::{ensure_available, Libs};
use crate::sys::libcuviddec_sys::CUcontext;
use crate::NvidiaError;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use std::ffi::c_uint;

    use crate::sys::libcuviddec_sys::{CUcontext, CUdevice, CUresult};

    pub type cuCtxCreate = unsafe extern "C" fn(*mut CUcontext, c_uint, CUdevice) -> CUresult;
    pub type cuCtxDestroy = unsafe extern "C" fn(CUcontext) -> CUresult;
    pub type cuCtxPushCurrent = unsafe extern "C" fn(CUcontext) -> CUresult;
    pub type cuCtxPopCurrent = unsafe extern "C" fn(*mut CUcontext) -> CUresult;
}

#[derive(Debug)]
pub struct CudaContext<'ctx> {
    context: Mutex<UnsafeCell<CUcontext>>,
    phantom: PhantomData<&'ctx CudaContext<'ctx>>,
}

impl<'a> Drop for CudaContext<'a> {
    fn drop(&mut self) {
        let Libs { lib_cuda, .. } =
            ensure_available().expect("How did we get here if lib_cuda isn't available?");

        let sym_cu_ctx_destroy =
            unsafe { lib_cuda.get::<cuCtxDestroy>(stringify!(cuCtxDestroy).as_bytes()) }
                .expect("cuCtxDestroy not found in lib_cuda");

        let context = self.context.lock().unwrap();
        unsafe {
            sym_cu_ctx_destroy(*context.get());
        }
    }
}

impl<'a> CudaContext<'a> {
    pub fn new(device: &CudaDevice) -> Result<Self, NvidiaError> {
        let Libs { lib_cuda, .. } = ensure_available()?;

        let sym_cu_ctx_create = get_sym!(lib_cuda, cuCtxCreate);
        let sym_cu_ctx_pop_current = get_sym!(lib_cuda, cuCtxPopCurrent);

        let mut cu_context: CUcontext = unsafe { zeroed() };

        call_cuda_sym!(sym_cu_ctx_create(&mut cu_context, 0, device.handle));
        call_cuda_sym!(sym_cu_ctx_pop_current(&mut cu_context));

        Ok(Self {
            context: Mutex::new(UnsafeCell::new(cu_context)),
            phantom: PhantomData::default(),
        })
    }

    /// Executes [f] while making sure the CUDA context is correctly applied to the GPU and cleaned
    /// up afterward.
    ///
    /// If `f` panics, the CUDA context is destroyed to release any resources and the panic
    /// propagated.
    pub fn with_ctx<F, T>(&self, f: F) -> Result<T, NvidiaError>
    where
        F: FnOnce() -> Result<T, NvidiaError> + UnwindSafe,
    {
        let context = self.context.lock().unwrap();

        let Libs { lib_cuda, .. } = ensure_available()?;

        let sym_cu_ctx_push_current = get_sym!(lib_cuda, cuCtxPushCurrent);
        let sym_cu_ctx_pop_current = get_sym!(lib_cuda, cuCtxPopCurrent);

        call_cuda_sym!(sym_cu_ctx_push_current(*context.get()));

        let f_result = match catch_unwind(f) {
            Ok(res) => res,
            Err(err) => {
                // we clean up the context on panic, so we don't leave the GPU in an
                // unexpected state before propagating the original panic value
                self.destroy(unsafe { *context.get() })?;

                panic_any(err);
            }
        };

        let ctx_ptr = context.get();
        // This won't change the actual context pointer, we just need
        // something to pass to the pop call
        call_cuda_sym!(sym_cu_ctx_pop_current(ctx_ptr));

        f_result
    }

    /// Executes [f] and supplies the CUDA context as a floating context instead of pushing it to
    /// the GPU directly.
    ///
    /// If `f` panics, the CUDA context is destroyed to release any resources and the panic
    /// propagated.
    pub fn with_floating_ctx<F, T>(&self, f: F) -> Result<T, NvidiaError>
    where
        F: FnOnce(&mut CUcontext) -> Result<T, NvidiaError> + UnwindSafe,
    {
        let context = self.context.lock().unwrap();

        match catch_unwind(|| f(unsafe { &mut *context.get() })) {
            Ok(res) => res,
            Err(err) => {
                self.destroy(unsafe { *context.get() })?;
                panic_any(err);
            }
        }
    }

    fn destroy(&self, ctx: CUcontext) -> Result<(), NvidiaError> {
        let Libs { lib_cuda, .. } = ensure_available()?;

        let sym_cu_ctx_destroy = get_sym!(lib_cuda, cuCtxDestroy);
        unsafe {
            sym_cu_ctx_destroy(ctx);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::device::enumerate_devices;
    use crate::dylib::is_cuda_loaded;

    use super::*;

    #[test]
    fn test_create_and_use_context() -> Result<(), Box<dyn Error>> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        assert!(!devices.is_empty());

        let context = CudaContext::new(devices.get(0).unwrap())?;

        context.with_ctx(|| {
            println!("We are using a CUcontext here");
            Ok(())
        })?;

        context.with_ctx(|| {
            println!("We are using a CUcontext here again");
            Ok(())
        })?;

        Ok(())
    }
}
