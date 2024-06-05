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
use std::ffi::CStr;
use std::path::PathBuf;
use std::ptr;
use std::sync::OnceLock;

use libloading::Symbol;
use thiserror::Error;

use crate::dylib::{ensure_available, Libs};
use crate::error::dylib_types::vaErrorStr;
use crate::sys::va::VAStatus;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use std::ffi::c_char;

    use crate::sys::va::VAStatus;

    pub type vaErrorStr = unsafe extern fn(error_status: VAStatus) -> *const c_char;
}

#[derive(Error, Debug)]
pub enum VaError {
    #[error("VA-API not available: {0}")]
    NotLoaded(#[from] &'static libloading::Error),
    #[error("Symbol not found in library: {0}")]
    SymbolNotFound(&'static str),
    #[error("Failed to enumerate devices: {0}")]
    FailedToEnumerateDevices(std::io::Error),
    #[error("Failed to open device at {0}: {1}")]
    FailedToOpenDevice(PathBuf, std::io::Error),
    #[error("Failed to get display for device at {0}")]
    FailedToGetDisplay(PathBuf),
    #[error("Operation failed: {0}")]
    OperationFailed(VAStatus),
    #[error("Operation failed: {0} ({1})")]
    OperationFailedT(String, VAStatus),
}

impl VaError {
    pub fn from_status(status: VAStatus) -> Result<VaError, VaError> {
        let Libs { libva, .. } = ensure_available()?;
        static SYM_VA_ERROR_STR: OnceLock<Symbol<vaErrorStr>> = OnceLock::new();

        let result_ptr = unsafe {
            SYM_VA_ERROR_STR.get_or_init(|| {
                libva.get::<vaErrorStr>(b"vaErrorStr\0")
                    .expect("vaErrorStr not found in libva.so")
            })(status)
        };

        if result_ptr == ptr::null() {
            Ok(VaError::OperationFailed(status))
        } else {
            let msg = unsafe { CStr::from_ptr(result_ptr) };
            Ok(VaError::OperationFailedT(msg.to_string_lossy().to_string(), status))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dylib::is_va_loaded;
    use crate::sys::va::VA_STATUS_ERROR_OPERATION_FAILED;

    use super::*;

    #[test]
    fn test_from_status() -> Result<(), VaError> {
        if !is_va_loaded() {
            eprintln!("libva.so not available");
            return Ok(());
        }

        let error = VaError::from_status(VA_STATUS_ERROR_OPERATION_FAILED.try_into().unwrap())?;

        dbg!(error);

        Ok(())
    }
}
