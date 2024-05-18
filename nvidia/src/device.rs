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
use std::ffi::{c_char, c_uint, CStr};

use uuid::Uuid;

use crate::call_cuda_sym;
use crate::device::dyn_types::{cuDeviceGet, cuDeviceGetCount, cuDeviceGetName, cuDeviceGetUuid};
use crate::dll::{ensure_available, Libs};
use crate::NvidiaError;
pub use crate::sys::libcuviddec_sys::CUdevice;
use crate::sys::libcuviddec_sys::CUuuid;

mod dyn_types {
    use std::ffi::{c_char, c_int, c_uint};

    use crate::sys::libcuviddec_sys::{CUdevice, CUresult, CUuuid};

    #[allow(non_camel_case_types, dead_code)]
    pub type cuDeviceGet = fn(*mut CUdevice, c_uint) -> CUresult;

    #[allow(non_camel_case_types, dead_code)]
    pub type cuDeviceGetCount = fn(*mut c_uint) -> CUresult;

    #[allow(non_camel_case_types, dead_code)]
    pub type cuDeviceGetName = fn(*mut c_char, c_int: c_int, CUdevice) -> CUresult;

    #[allow(non_camel_case_types, dead_code)]
    pub type cuDeviceGetUuid = fn(*mut CUuuid, CUdevice) -> CUresult;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CudaDevice {
    pub handle: CUdevice,
    pub name: String,
    pub uuid: Uuid,
}

pub fn enumerate_devices() -> Result<Vec<CudaDevice>, NvidiaError> {
    let Libs { lib_cuda, .. } = ensure_available()?;

    let sym_cu_device_get: libloading::Symbol<cuDeviceGet> = unsafe { lib_cuda.get(b"cuDeviceGet").unwrap() };
    let sym_cu_device_get_count: libloading::Symbol<cuDeviceGetCount> = unsafe { lib_cuda.get(b"cuDeviceGetCount").unwrap() };
    let sym_cu_device_get_name: libloading::Symbol<cuDeviceGetName> = unsafe { lib_cuda.get(b"cuDeviceGetName").unwrap() };
    let sym_cu_device_get_uuid: libloading::Symbol<cuDeviceGetUuid> = unsafe { lib_cuda.get(b"cuDeviceGetUuid").unwrap() };

    let mut devices = Vec::new();

    let device_count = {
        let mut count = unsafe { std::mem::zeroed::<c_uint>() };
        call_cuda_sym!(sym_cu_device_get_count(&mut count));

        count
    };

    for ordinal in 0..device_count {
        let mut cu_device = unsafe { std::mem::zeroed::<CUdevice>() };
        call_cuda_sym!(sym_cu_device_get(&mut cu_device, ordinal));

        let cu_name_buffer = [0u8; 64];
        call_cuda_sym!(sym_cu_device_get_name(cu_name_buffer.as_ptr() as *mut c_char, 64, cu_device));

        let cu_name_raw = CStr::from_bytes_until_nul(&cu_name_buffer).unwrap();

        let mut cu_uuid_buffer = unsafe { std::mem::zeroed::<CUuuid>() };
        call_cuda_sym!(sym_cu_device_get_uuid(&mut cu_uuid_buffer, cu_device));

        let uuid = Uuid::from_slice(
            unsafe {
                std::slice::from_raw_parts(
                    cu_uuid_buffer.bytes.as_ptr() as *const u8,
                    16,
                )
            }
        ).unwrap();

        devices.push(
            CudaDevice {
                handle: cu_device,
                name: cu_name_raw.to_string_lossy().to_string(),
                uuid,
            }
        )
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use crate::dll::is_cuda_loaded;

    use super::*;

    #[test]
    fn test_enumerate_devices() -> Result<(), NvidiaError> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        dbg!(&devices);

        assert!(!devices.is_empty());

        Ok(())
    }
}
