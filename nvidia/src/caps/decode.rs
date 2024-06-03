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

use std::ffi::c_uint;
use std::mem::zeroed;

use dylib_types::*;

use crate::dylib::{ensure_available, Libs};
use crate::NvidiaError;
use crate::sys::libcuviddec_sys::{cudaVideoChromaFormat, cudaVideoCodec, CUVIDDECODECAPS};

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use crate::sys::libcuviddec_sys::{CUresult, CUVIDDECODECAPS};

    pub type cuvidGetDecoderCaps = unsafe extern fn(*mut CUVIDDECODECAPS) -> CUresult;
}

#[derive(Debug)]
pub struct CudaDecodeSpec {
    pub codec_type: cudaVideoCodec,
    pub chroma_format: cudaVideoChromaFormat,
    pub bit_depth: u8,
}

#[derive(Debug)]
pub struct CudaDecodeCapabilities {
    pub spec: CudaDecodeSpec,
    pub max_width: c_uint,
    pub max_height: c_uint,
}

/// Retrieves the decode capabilities for the specified [specs]. The resulting vector only
/// contains [CudaDecodeCapabilities] instances where decoding is supported, so the number of
/// results may be lower than the number of specs.
///
/// This function requires an applied [crate::context::CudaContext], so make sure to surround any 
/// call to this function with [crate::context::CudaContext::with_ctx].
pub fn get_decode_capabilities(specs: Vec<CudaDecodeSpec>) -> Result<Vec<CudaDecodeCapabilities>, NvidiaError> {
    let Libs { lib_cuviddec, .. } = ensure_available()?;

    let sym_cuvid_get_decoder_caps = get_sym!(lib_cuviddec, cuvidGetDecoderCaps);

    let mut result = Vec::new();

    for spec in specs.into_iter() {
        let mut cuvid_decode_caps: CUVIDDECODECAPS = unsafe { zeroed() };
        cuvid_decode_caps.eCodecType = spec.codec_type;
        cuvid_decode_caps.eChromaFormat = spec.chroma_format;
        cuvid_decode_caps.nBitDepthMinus8 = (spec.bit_depth - 8).into();

        call_cuda_sym!(sym_cuvid_get_decoder_caps(&mut cuvid_decode_caps));

        if cuvid_decode_caps.bIsSupported != 0 {
            result.push(CudaDecodeCapabilities {
                spec,
                max_width: cuvid_decode_caps.nMaxWidth,
                max_height: cuvid_decode_caps.nMaxHeight,
            })
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::context::CudaContext;
    use crate::device::enumerate_devices;
    use crate::dylib::is_cuda_loaded;
    use crate::sys::libcuviddec_sys::{cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420, cudaVideoCodec_enum_cudaVideoCodec_H264};

    use super::*;

    #[test]
    fn test_get_decode_capabilities() -> Result<(), NvidiaError> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so is not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        assert!(!devices.is_empty());

        let context = CudaContext::new(devices.get(0).unwrap())?;

        let caps = context.with_ctx(|| {
            get_decode_capabilities(vec![CudaDecodeSpec {
                codec_type: cudaVideoCodec_enum_cudaVideoCodec_H264,
                chroma_format: cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420,
                bit_depth: 8,
            }])
        })?;

        dbg!(&caps);
        assert_eq!(1, caps.len());

        Ok(())
    }
}
