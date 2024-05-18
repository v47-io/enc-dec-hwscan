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
/*
#[derive(Clone, Copy, Debug)]
pub struct CuvidGetDecoderCapsInput {
    pub codec_type: cudaVideoCodec,
    pub chroma_format: cudaVideoChromaFormat,
    pub bit_depth: u8,
}

pub fn cuvid_get_decoder_caps(inputs: &[CuvidGetDecoderCapsInput]) -> Result<Vec<CUVIDDECODECAPS>, NvidiaError> {
    ensure_available()?;

    let sym_cuvid_get_decoder_caps: libloading::Symbol<cuvidGetDecoderCaps> = unsafe {
        (*LIBCUVIDDEC)
            ?.get(b"cuvidGetDecoderCaps")
            .expect("cuvidGetDecoderCaps not found in libnvcuvid.so")
    };

    Ok(inputs.iter().filter_map(|input| {
        let mut caps = unsafe { std::mem::zeroed::<CUVIDDECODECAPS>() };
        caps.eCodecType = input.codec_type;
        caps.eChromaFormat = input.chroma_format;
        caps.nBitDepthMinus8 = (input.bit_depth - 8).into();

        let result = sym_cuvid_get_decoder_caps(&mut caps);

        if result == cudaError_enum_CUDA_SUCCESS {
            if caps.bIsSupported != 0 {
                Some(caps)
            } else {
                None
            }
        } else {
            eprintln!("Failed to get decoder capabilities: errno = {}", result);
            None
        }
    }).collect())
}
 */
