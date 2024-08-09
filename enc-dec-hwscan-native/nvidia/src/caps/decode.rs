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

use std::collections::HashMap;
use std::mem::zeroed;

use common::{Chroma, Codec, CodecDetails, ColorDepth, DecodingSpec};
use dylib_types::*;

use crate::dylib::{ensure_available, Libs};
use crate::sys::libcuviddec_sys::{
    cudaVideoChromaFormat, cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome, cudaVideoCodec,
    cudaVideoCodec_enum_cudaVideoCodec_AV1, cudaVideoCodec_enum_cudaVideoCodec_H264,
    cudaVideoCodec_enum_cudaVideoCodec_HEVC, cudaVideoCodec_enum_cudaVideoCodec_MPEG1,
    cudaVideoCodec_enum_cudaVideoCodec_MPEG2, cudaVideoCodec_enum_cudaVideoCodec_MPEG4,
    cudaVideoCodec_enum_cudaVideoCodec_VC1, cudaVideoCodec_enum_cudaVideoCodec_VP8,
    cudaVideoCodec_enum_cudaVideoCodec_VP9, CUVIDDECODECAPS,
};
use crate::NvidiaError;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use crate::sys::libcuviddec_sys::{CUresult, CUVIDDECODECAPS};

    pub type cuvidGetDecoderCaps = unsafe extern "C" fn(*mut CUVIDDECODECAPS) -> CUresult;
}

const CUDA_CODECS: [cudaVideoCodec; 9] = [
    cudaVideoCodec_enum_cudaVideoCodec_MPEG1,
    cudaVideoCodec_enum_cudaVideoCodec_MPEG2,
    cudaVideoCodec_enum_cudaVideoCodec_MPEG4,
    cudaVideoCodec_enum_cudaVideoCodec_VC1,
    cudaVideoCodec_enum_cudaVideoCodec_H264,
    cudaVideoCodec_enum_cudaVideoCodec_HEVC,
    cudaVideoCodec_enum_cudaVideoCodec_VP8,
    cudaVideoCodec_enum_cudaVideoCodec_VP9,
    cudaVideoCodec_enum_cudaVideoCodec_AV1,
];

const CUDA_CHROMA_FORMATS: [cudaVideoChromaFormat; 4] = [
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422,
    cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444,
];

const CUDA_BIT_DEPTHS: [u8; 3] = [8, 10, 12];

#[derive(Debug)]
pub struct CudaDecodeSpec {
    codec_type: cudaVideoCodec,
    chroma_format: cudaVideoChromaFormat,
    bit_depth: u8,
}

impl CudaDecodeSpec {
    pub fn all() -> Vec<CudaDecodeSpec> {
        CUDA_CODECS
            .iter()
            .flat_map(|&codec_type| {
                CUDA_CHROMA_FORMATS.iter().flat_map(move |&chroma_format| {
                    CUDA_BIT_DEPTHS
                        .iter()
                        .map(move |&bit_depth| CudaDecodeSpec {
                            codec_type,
                            chroma_format,
                            bit_depth,
                        })
                })
            })
            .collect()
    }
}

/// This function requires an applied [crate::context::CudaContext], so make sure to surround any
/// call to this function with [crate::context::CudaContext::with_ctx].
pub fn get_decode_capabilities(specs: &[CudaDecodeSpec]) -> Result<Vec<CodecDetails>, NvidiaError> {
    let Libs { lib_cuviddec, .. } = ensure_available()?;

    let sym_cuvid_get_decoder_caps = get_sym!(lib_cuviddec, cuvidGetDecoderCaps);

    let mut result: HashMap<Codec, Vec<DecodingSpec>> = HashMap::new();

    for spec in specs.into_iter() {
        let mut cuvid_decode_caps: CUVIDDECODECAPS = unsafe { zeroed() };
        cuvid_decode_caps.eCodecType = spec.codec_type;
        cuvid_decode_caps.eChromaFormat = spec.chroma_format;
        cuvid_decode_caps.nBitDepthMinus8 = (spec.bit_depth - 8).into();

        call_cuda_sym!(sym_cuvid_get_decoder_caps(&mut cuvid_decode_caps));

        if cuvid_decode_caps.bIsSupported != 0 {
            result
                .entry(map_codec_type(spec.codec_type))
                .or_default()
                .push(DecodingSpec {
                    chroma: map_chroma_format(spec.chroma_format),
                    color_depth: map_bit_depth(spec.bit_depth),
                    max_width: cuvid_decode_caps.nMaxWidth,
                    max_height: cuvid_decode_caps.nMaxHeight,
                });
        }
    }

    Ok(result
        .into_iter()
        .map(|(codec, specs)| CodecDetails::new(codec, specs, vec![]))
        .collect())
}

fn map_codec_type(cuda_video_codec: cudaVideoCodec) -> Codec {
    #[allow(non_upper_case_globals)]
    match cuda_video_codec {
        cudaVideoCodec_enum_cudaVideoCodec_MPEG1 => Codec::Mpeg1,
        cudaVideoCodec_enum_cudaVideoCodec_MPEG2 => Codec::Mpeg2,
        cudaVideoCodec_enum_cudaVideoCodec_MPEG4 => Codec::Mpeg4,
        cudaVideoCodec_enum_cudaVideoCodec_VC1 => Codec::Vc1,
        cudaVideoCodec_enum_cudaVideoCodec_H264 => Codec::H264,
        cudaVideoCodec_enum_cudaVideoCodec_HEVC => Codec::Hevc,
        cudaVideoCodec_enum_cudaVideoCodec_VP8 => Codec::Vp8,
        cudaVideoCodec_enum_cudaVideoCodec_VP9 => Codec::Vp9,
        cudaVideoCodec_enum_cudaVideoCodec_AV1 => Codec::Av1,
        _ => unreachable!(),
    }
}

fn map_chroma_format(cuda_chroma_format: cudaVideoChromaFormat) -> Chroma {
    #[allow(non_upper_case_globals)]
    match cuda_chroma_format {
        cudaVideoChromaFormat_enum_cudaVideoChromaFormat_Monochrome => Chroma::Monochrome,
        cudaVideoChromaFormat_enum_cudaVideoChromaFormat_420 => Chroma::Yuv420,
        cudaVideoChromaFormat_enum_cudaVideoChromaFormat_422 => Chroma::Yuv422,
        cudaVideoChromaFormat_enum_cudaVideoChromaFormat_444 => Chroma::Yuv444,
        _ => unreachable!(),
    }
}

fn map_bit_depth(bit_depth: u8) -> ColorDepth {
    match bit_depth {
        8 => ColorDepth::Bit8,
        10 => ColorDepth::Bit10,
        12 => ColorDepth::Bit12,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::context::CudaContext;
    use crate::device::enumerate_devices;
    use crate::dylib::is_cuda_loaded;

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

        let caps = context.with_ctx(|| get_decode_capabilities(&CudaDecodeSpec::all()))?;

        dbg!(&caps);
        assert!(!caps.is_empty());

        Ok(())
    }
}
