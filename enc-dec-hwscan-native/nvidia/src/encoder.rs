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

use std::alloc::{alloc_zeroed, Layout};
use std::ffi::{c_uint, c_void};
use std::mem::zeroed;
use std::ptr;

use uuid::Uuid;

use dylib_types::*;

use crate::dylib::{ensure_available, Libs};
use crate::sys::libcuviddec_sys::CUcontext;
use crate::sys::libnv_encode_api_sys::{
    GUID, NV_ENCODE_API_FUNCTION_LIST, NV_ENC_CAPS_PARAM, NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    _NVENCSTATUS_NV_ENC_SUCCESS, _NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA,
};
use crate::NvidiaError;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use crate::sys::libnv_encode_api_sys::{NVENCSTATUS, NV_ENCODE_API_FUNCTION_LIST};

    pub type NvEncodeAPICreateInstance =
        unsafe extern "C" fn(*mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS;
}

const NVENCAPI_MAJOR_VERSION: u32 = 7;
const NVENCAPI_MINOR_VERSION: u32 = 0;

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
            let nv_enc_destroy_encoder = self
                .encode_api
                .nvEncDestroyEncoder
                .expect("no nvEncDestroyEncoder? weird...");

            let nvencstatus = nv_enc_destroy_encoder(self.handle);

            if cfg!(test) {
                // this only matters to me during tests, outside we'll just have to live with
                // failures and hope the Nvidia driver can handle it
                assert_eq!(_NVENCSTATUS_NV_ENC_SUCCESS, nvencstatus);
            }
        }
    }
}

#[allow(clippy::crate_in_macro_def)]
macro_rules! call_encoder_fn {
    ($encoder: expr, $func: ident ($($arg: expr),*)) => {{
        use crate::NvidiaError;
        use crate::sys::libnv_encode_api_sys::_NVENCSTATUS_NV_ENC_SUCCESS;

        match $encoder.encode_api.$func {
            Some(func) => unsafe {
                let nvencstatus = func($encoder.handle, $($arg),*);
                if nvencstatus != _NVENCSTATUS_NV_ENC_SUCCESS {
                    return Err(NvidiaError::OperationFailed(nvencstatus));
                }
            },
            None => return Err(NvidiaError::NvEncFunctionNotAvailable(stringify!($func)))
        }
    }}
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
            None => {
                return Err(NvidiaError::NvEncFunctionNotAvailable(stringify!(
                    nvEncOpenEncodeSessionEx
                )))
            }
        }

        Ok(NvEncoder {
            encode_api,
            handle,
            _context: context,
        })
    }

    pub fn get_encode_guids(&self) -> Result<Vec<Uuid>, NvidiaError> {
        let mut guid_count = 0u32;
        call_encoder_fn!(self, nvEncGetEncodeGUIDCount(&mut guid_count));

        let guids_ptr = alloc_guid_array(guid_count);
        call_encoder_fn!(
            self,
            nvEncGetEncodeGUIDs(guids_ptr, guid_count, &mut guid_count)
        );

        Ok(make_uuid_vec(guids_ptr, guid_count as usize)?)
    }

    pub fn get_encode_profile_guids(&self, codec_guid: &Uuid) -> Result<Vec<Uuid>, NvidiaError> {
        let mut guid_count = 0u32;
        call_encoder_fn!(
            self,
            nvEncGetEncodeProfileGUIDCount(codec_guid.into(), &mut guid_count)
        );

        let guids_ptr = alloc_guid_array(guid_count);
        call_encoder_fn!(
            self,
            nvEncGetEncodeProfileGUIDs(codec_guid.into(), guids_ptr, guid_count, &mut guid_count)
        );

        Ok(make_uuid_vec(guids_ptr, guid_count as usize)?)
    }

    pub fn get_encode_caps(&self, codec_guid: &Uuid, cap: u32) -> Result<i32, NvidiaError> {
        let mut enc_caps_param: NV_ENC_CAPS_PARAM = unsafe { zeroed() };
        enc_caps_param.version = nvencapi_struct_version(1);
        enc_caps_param.capsToQuery = cap as c_uint;

        let mut caps_val = 0i32;
        call_encoder_fn!(
            self,
            nvEncGetEncodeCaps(codec_guid.into(), &mut enc_caps_param, &mut caps_val)
        );

        Ok(caps_val)
    }
}

fn alloc_guid_array(guid_count: u32) -> *mut GUID {
    let guid_raw_layout = Layout::array::<GUID>(guid_count as usize).unwrap();

    unsafe { alloc_zeroed(guid_raw_layout) as *mut GUID }
}

fn make_uuid_vec(guids_raw: *mut GUID, guid_count: usize) -> Result<Vec<Uuid>, NvidiaError> {
    let guids_raw = unsafe { Vec::from_raw_parts(guids_raw, guid_count, guid_count) };

    let mut result = Vec::new();

    for guid_raw in guids_raw.into_iter() {
        result.push(guid_raw.try_into()?);
    }

    Ok(result)
}

pub mod guid {
    use std::mem::zeroed;

    use uuid::Uuid;

    use crate::sys::libnv_encode_api_sys::GUID;

    pub const CODEC_H264: Uuid = Uuid::from_bytes([
        0x6b, 0xc8, 0x27, 0x62, 0x4e, 0x63, 0x4c, 0xa4, 0xaa, 0x85, 0x1e, 0x50, 0xf3, 0x21, 0xf6,
        0xbf,
    ]);
    pub const CODEC_HEVC: Uuid = Uuid::from_bytes([
        0x79, 0x0c, 0xdc, 0x88, 0x45, 0x22, 0x4d, 0x7b, 0x94, 0x25, 0xbd, 0xa9, 0x97, 0x5f, 0x76,
        0x03,
    ]);
    pub const CODEC_AV1: Uuid = Uuid::from_bytes([
        0x0a, 0x35, 0x22, 0x89, 0x0a, 0xa7, 0x47, 0x59, 0x86, 0x2d, 0x5d, 0x15, 0xcd, 0x16, 0xd2,
        0x54,
    ]);

    pub const H264_PROFILE_BASELINE: Uuid = Uuid::from_bytes([
        0x07, 0x27, 0xbc, 0xaa, 0x78, 0xc4, 0x4c, 0x83, 0x8c, 0x2f, 0xef, 0x3d, 0xff, 0x26, 0x7c,
        0x6a,
    ]);
    pub const H264_PROFILE_MAIN: Uuid = Uuid::from_bytes([
        0x60, 0xb5, 0xc1, 0xd4, 0x67, 0xfe, 0x47, 0x90, 0x94, 0xd5, 0xc4, 0x72, 0x6d, 0x7b, 0x6e,
        0x6d,
    ]);
    pub const H264_PROFILE_HIGH: Uuid = Uuid::from_bytes([
        0xe7, 0xcb, 0xc3, 0x09, 0x4f, 0x7a, 0x4b, 0x89, 0xaf, 0x2a, 0xd5, 0x37, 0xc9, 0x2b, 0xe3,
        0x10,
    ]);
    pub const H264_PROFILE_HIGH_444: Uuid = Uuid::from_bytes([
        0x7a, 0xc6, 0x63, 0xcb, 0xa5, 0x98, 0x49, 0x60, 0xb8, 0x44, 0x33, 0x9b, 0x26, 0x1a, 0x7d,
        0x52,
    ]);

    pub const HEVC_PROFILE_MAIN: Uuid = Uuid::from_bytes([
        0xb5, 0x14, 0xc3, 0x9a, 0xb5, 0x5b, 0x40, 0xfa, 0x87, 0x8f, 0xf1, 0x25, 0x3b, 0x4d, 0xfd,
        0xec,
    ]);
    pub const HEVC_PROFILE_MAIN10: Uuid = Uuid::from_bytes([
        0xfa, 0x4d, 0x2b, 0x6c, 0x3a, 0x5b, 0x41, 0x1a, 0x80, 0x18, 0x0a, 0x3f, 0x5e, 0x3c, 0x9b,
        0xe5,
    ]);

    pub const AV1_PROFILE_MAIN: Uuid = Uuid::from_bytes([
        0x5f, 0x2a, 0x39, 0xf5, 0xf1, 0x4e, 0x4f, 0x95, 0x9a, 0x9e, 0xb7, 0x6d, 0x56, 0x8f, 0xcf,
        0x97,
    ]);

    impl TryFrom<GUID> for Uuid {
        type Error = uuid::Error;

        fn try_from(value: GUID) -> Result<Self, Self::Error> {
            let data1_bytes: [u8; 4] = value.Data1.to_be_bytes();
            let data2_bytes: [u8; 2] = value.Data2.to_be_bytes();
            let data3_bytes: [u8; 2] = value.Data3.to_be_bytes();

            let full_bytes_vec: Vec<&[u8]> =
                vec![&data1_bytes, &data2_bytes, &data3_bytes, &value.Data4];
            let full_bytes: Vec<u8> = full_bytes_vec.concat();

            Uuid::from_slice(&full_bytes)
        }
    }

    impl From<&Uuid> for GUID {
        fn from(value: &Uuid) -> Self {
            let mut guid: GUID = unsafe { zeroed() };

            let fields = value.as_fields();
            guid.Data1 = fields.0;
            guid.Data2 = fields.1;
            guid.Data3 = fields.2;

            let mut bytes: [u8; 8] = unsafe { zeroed() };
            bytes.copy_from_slice(fields.3);

            guid.Data4 = bytes;

            guid
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::context::CudaContext;
    use crate::device::enumerate_devices;
    use crate::dylib::is_cuda_loaded;

    use super::*;

    #[test]
    fn test_get_encode_guids() -> Result<(), NvidiaError> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        assert!(!devices.is_empty());

        let context = CudaContext::new(devices.get(0).unwrap())?;

        context.with_floating_ctx(|context| {
            let encoder = NvEncoder::new(context)?;
            let encode_guids = encoder.get_encode_guids()?;

            dbg!(&encode_guids);
            assert!(!encode_guids.is_empty());

            let encode_profile_guids =
                encoder.get_encode_profile_guids(encode_guids.get(0).unwrap())?;

            dbg!(&encode_profile_guids);
            assert!(!encode_profile_guids.is_empty());

            Ok(())
        })?;

        Ok(())
    }
}
