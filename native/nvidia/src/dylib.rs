/*
 * Copyright (C) 2024 Alex Katlein <dev@vemilyus.com>
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
use std::sync::Mutex;

use lazy_static::lazy_static;
use libloading::{Error, Library};

use dylib_types::*;

use crate::sys::libcuviddec_sys::cudaError_enum_CUDA_SUCCESS;
use crate::NvidiaError;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use std::ffi::c_uint;

    use crate::sys::libcuviddec_sys::CUresult;

    pub type cuInit = unsafe extern "C" fn(c_uint: c_uint) -> CUresult;
}

lazy_static! {
    static ref _LIBCUDA_RAW: Result<Library, Error> = unsafe { Library::new("libcuda.so") };
    pub static ref LIBCUDA: Result<&'static Library, &'static Error> = _LIBCUDA_RAW.as_ref();
    static ref _LIBCUVIDDEC_RAW: Result<Library, Error> = unsafe { Library::new("libnvcuvid.so") };
    pub static ref LIBCUVIDDEC: Result<&'static Library, &'static Error> =
        _LIBCUVIDDEC_RAW.as_ref();
    static ref _LIBNV_ENCODE_RAW: Result<Library, Error> =
        unsafe { Library::new("libnvidia-encode.so") };
    pub static ref LIBNV_ENCODE: Result<&'static Library, &'static Error> =
        _LIBNV_ENCODE_RAW.as_ref();
    pub static ref CUDA_INITIALIZED: Mutex<bool> = Mutex::new(false);
    pub static ref CUDA_INIT_FAILED: Mutex<bool> = Mutex::new(false);
}

#[cfg(test)]
pub(crate) fn is_cuda_loaded() -> bool {
    (*LIBCUDA).is_ok()
}

#[derive(Copy, Clone)]
pub struct Libs {
    pub lib_cuda: &'static Library,
    pub lib_cuviddec: &'static Library,
    pub lib_nv_encode: &'static Library,
}

pub fn ensure_available() -> Result<Libs, NvidiaError> {
    cuda_init()?;

    Ok(Libs {
        lib_cuda: (*LIBCUDA)?,
        lib_cuviddec: (*LIBCUVIDDEC)?,
        lib_nv_encode: (*LIBNV_ENCODE)?,
    })
}

pub fn cuda_init() -> Result<(), NvidiaError> {
    let mut init_handle = CUDA_INITIALIZED.lock().unwrap();

    if !*init_handle {
        let sym_cu_init: libloading::Symbol<cuInit> = unsafe {
            (*LIBCUDA)?
                .get(b"cuInit\0")
                .expect("cuInit not found in libcuda.so")
        };

        let init_result = unsafe { sym_cu_init(0) };
        if init_result != cudaError_enum_CUDA_SUCCESS {
            let mut init_failed_handle = CUDA_INIT_FAILED.lock().unwrap();
            *init_failed_handle = true;

            return Err(NvidiaError::OperationFailed(init_result));
        }
    }

    *init_handle = true;
    Ok(())
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! call_cuda_sym {
    ($call: expr) => {{
        use crate::NvidiaError;

        use crate::sys::libcuviddec_sys::cudaError_enum_CUDA_SUCCESS;

        #[allow(clippy::macro_metavars_in_unsafe)]
        let curesult = unsafe { $call };
        if curesult != cudaError_enum_CUDA_SUCCESS {
            return Err(NvidiaError::OperationFailed(curesult));
        }
    }};
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! get_sym {
    ($lib_var:expr, $sym_name:ident) => {{
        use crate::NvidiaError;

        unsafe {
            match $lib_var.get::<$sym_name>(stringify!($sym_name).as_bytes()) {
                Ok(sym) => sym,
                Err(_) => return Err(NvidiaError::SymbolNotFound(stringify!($sym_name))),
            }
        }
    }};
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! get_sym_opt {
    ($lib_var:expr, $sym_name:ident) => {{
        unsafe {
            match $lib_var.get::<$sym_name>(stringify!($sym_name).as_bytes()) {
                Ok(sym) => Some(sym),
                Err(_) => None,
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuda_is_initialized() {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return;
        }

        assert!(cuda_init().is_ok())
    }
}
