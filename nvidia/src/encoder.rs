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

use std::ffi::c_void;
use std::mem::zeroed;
use std::ptr;

use dylib_types::*;

use crate::dll::{ensure_available, Libs};
use crate::NvidiaError;
use crate::sys::libcuviddec_sys::CUcontext;
use crate::sys::libnv_encode_api_sys::{_NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA, _NVENCSTATUS_NV_ENC_SUCCESS, NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS, NV_ENCODE_API_FUNCTION_LIST};

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use crate::sys::libnv_encode_api_sys::{NV_ENCODE_API_FUNCTION_LIST, NVENCSTATUS};

    pub type NvEncodeAPICreateInstance = unsafe extern fn(*mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS;
}

const NVENCAPI_MAJOR_VERSION: u32 = 12;
const NVENCAPI_MINOR_VERSION: u32 = 2;

const NVENCAPI_VERSION: u32 = NVENCAPI_MAJOR_VERSION | (NVENCAPI_MINOR_VERSION << 24);

pub const fn nvencapi_struct_version(version: u32) -> u32 {
    NVENCAPI_VERSION | (version << 16) | (0x7 << 28)
}

fn encode_api<'a>() -> Result<NV_ENCODE_API_FUNCTION_LIST, NvidiaError> {
    let Libs { lib_nv_encode, .. } = ensure_available()?;
    let sym_nv_encode_api_create_instance = get_sym!(lib_nv_encode, NvEncodeAPICreateInstance);

    let mut instance_ptr: NV_ENCODE_API_FUNCTION_LIST = unsafe { zeroed() };
    instance_ptr.version = nvencapi_struct_version(2);

    let nvencstatus = unsafe { sym_nv_encode_api_create_instance(&mut instance_ptr) };
    if nvencstatus != _NVENCSTATUS_NV_ENC_SUCCESS {
        return Err(NvidiaError::OperationFailed(nvencstatus));
    }

    Ok(instance_ptr)
}

#[derive(Debug)]
pub struct NvEncoder<'enc> {
    encode_api: NV_ENCODE_API_FUNCTION_LIST,
    handle: *mut c_void,
    _context: &'enc CUcontext,
}

impl<'a> Drop for NvEncoder<'a> {
    fn drop(&mut self) {
        unsafe {
            let nv_enc_destroy_encoder =
                self.encode_api
                    .nvEncDestroyEncoder
                    .expect("no nvEncDestroyEncoder? weird...");

            let nvencstatus = nv_enc_destroy_encoder(self.handle);
            assert_eq!(_NVENCSTATUS_NV_ENC_SUCCESS, nvencstatus);
        }
    }
}

impl<'a> NvEncoder<'a> {
    pub fn new(context: &'a mut CUcontext) -> Result<NvEncoder<'a>, NvidiaError> {
        let encode_api = encode_api()?;

        let mut handle = ptr::null_mut();

        let mut open_params_ptr: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS = unsafe { zeroed() };
        open_params_ptr.version = nvencapi_struct_version(1);
        open_params_ptr.deviceType = _NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA;
        open_params_ptr.device = *context as *mut c_void;
        open_params_ptr.apiVersion = NVENCAPI_VERSION;

        match encode_api.nvEncOpenEncodeSessionEx {
            Some(func) => unsafe {
                let nvencstatus = func(&mut open_params_ptr, &mut handle);
                if nvencstatus != _NVENCSTATUS_NV_ENC_SUCCESS {
                    // inlined the macro because of this, nvEncodeAPI.h requires us to destroy
                    // the encoder even when the initial creation failed
                    encode_api.nvEncDestroyEncoder.unwrap()(handle);
                    return Err(NvidiaError::OperationFailed(nvencstatus));
                }
            },
            None => return Err(NvidiaError::NvEncFunctionNotAvailable(stringify!(nvEncOpenEncodeSessionEx)))
        }

        Ok(NvEncoder {
            encode_api,
            handle,
            _context: context,
        })
    }
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! call_nvenc_api {
    ($api: expr, $func: ident ($($arg: expr),*)) => {{
        use crate::error::NvidiaError;
        use crate::sys::libnv_encode_api_sys::_NVENCSTATUS_NV_ENC_SUCCESS;

        match $api.$func {
            Some(func) => unsafe {
                let nvencstatus = func($($arg),*);
                if nvencstatus != _NVENCSTATUS_NV_ENC_SUCCESS {
                    return Err(NvidiaError::OperationFailed(nvencstatus));
                }
            },
            None => return Err(NvidiaError::NvEncFunctionNotAvailable(stringify!($func)))
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::context::CudaContext;
    use crate::device::enumerate_devices;
    use crate::dll::is_cuda_loaded;

    use super::*;

    #[test]
    fn test_create_encoder() -> Result<(), NvidiaError> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        assert!(!devices.is_empty());

        let context = CudaContext::new(devices.get(0).unwrap())?;

        context.with_floating_ctx(|context| {
            let _ = NvEncoder::new(context)?;

            Ok(())
        })?;

        Ok(())
    }
}
