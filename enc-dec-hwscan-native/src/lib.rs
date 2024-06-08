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
use std::panic::catch_unwind;

use ::nvidia::NvidiaError;
use ::vaapi::VaError;
pub use common::*;

use crate::error::ErrorCode;
use crate::nvidia::get_nvidia_devices;
use crate::vaapi::get_vaapi_devices;

mod nvidia;
mod error;
mod vaapi;

#[no_mangle]
pub unsafe extern "C" fn free_devices(ptr: *mut EncDecDevices) {
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn scan_devices(result: *mut *mut EncDecDevices) -> ErrorCode {
    catch_unwind(|| {
        let (nvidia_devices, nvidia_done) = match get_nvidia_devices() {
            Ok(devs) => (devs, true),
            Err(err) => {
                if let NvidiaError::NotLoaded(_) = err {
                    (vec![], false)
                } else {
                    eprintln!("enc-dec-hwscan error: {}", err.to_string());
                    return map_nvidia_error_code(err);
                }
            }
        };

        let vaapi_devices = match get_vaapi_devices(nvidia_done) {
            Ok(devs) => devs,
            Err(err) => {
                if let VaError::NotLoaded(_) = err {
                    vec![]
                } else {
                    eprintln!("enc-dec-hwscan error: {}", err.to_string());
                    return map_vaapi_error_code(err);
                }
            }
        };

        let all_devices =
            nvidia_devices.into_iter()
                .chain(vaapi_devices.into_iter())
                .collect();

        let devices = Box::new(EncDecDevices::new(all_devices));

        // make sure this is done last and only if errno is going to be 0
        *result = Box::into_raw(devices);
        ErrorCode::Success
    }).unwrap_or_else(|err| {
        eprintln!("Critical error in enc_dec_hwscan::detect_devices: {:?}", err);
        ErrorCode::CriticalError
    })
}

fn map_nvidia_error_code(error: NvidiaError) -> ErrorCode {
    match error {
        NvidiaError::NotLoaded(_) => ErrorCode::Success,
        NvidiaError::OperationFailed(_) => ErrorCode::OperationFailed,
        NvidiaError::SymbolNotFound(_) => ErrorCode::DriverFailure,
        NvidiaError::NvEncFunctionNotAvailable(_) => ErrorCode::DriverFailure,
        NvidiaError::FailedToConvertUuid(_) => ErrorCode::ConversionFailed,
        NvidiaError::FailedToConvertResult(_) => ErrorCode::ConversionFailed
    }
}

fn map_vaapi_error_code(error: VaError) -> ErrorCode {
    match error {
        VaError::NotLoaded(_) => ErrorCode::Success,
        VaError::SymbolNotFound(_) => ErrorCode::DriverFailure,
        VaError::FailedToEnumerateDevices(_) => ErrorCode::DriverFailure,
        VaError::FailedToOpenDevice(_, _) => ErrorCode::DriverFailure,
        VaError::FailedToGetDisplay(_) => ErrorCode::DriverFailure,
        VaError::OperationFailed(_, _) => ErrorCode::OperationFailed
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_detect_devices() {
        unsafe {
            let mut target = ptr::null_mut::<EncDecDevices>();

            assert_eq!(ErrorCode::Success, scan_devices(&mut target));

            dbg!(&*target);

            free_devices(target);
        }
    }
}
