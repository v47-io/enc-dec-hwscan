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
use uuid::Uuid;

use common::{Codec, EncodeProfile};

use crate::encoder::guid::{
    AV1_PROFILE_MAIN, CODEC_AV1, CODEC_H264,
    CODEC_HEVC, H264_PROFILE_BASELINE, H264_PROFILE_HIGH,
    H264_PROFILE_HIGH_444, H264_PROFILE_MAIN, HEVC_PROFILE_MAIN,
    HEVC_PROFILE_MAIN10,
};
use crate::encoder::NvEncoder;
use crate::NvidiaError;
use crate::sys::libnv_encode_api_sys::{
    _NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MAX,
    _NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES,
    _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE,
    _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE,
    _NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX,
};

#[derive(Debug)]
pub struct NvEncodeCapabilities {
    pub codec: Codec,
    pub profiles: Vec<EncodeProfile>,
    pub max_width: usize,
    pub max_height: usize,
    pub ten_bit_encode_supported: bool,
    pub b_frames_supported: bool,
    pub yuv_444_encode_supported: bool,
}

const IGNORED_PROFILES: [u32; 3] = [0x40847bf5, 0xbfd6f8e7, 0x51ec32b5];

pub fn get_encode_capabilities(encoder: &NvEncoder) -> Result<Vec<NvEncodeCapabilities>, NvidiaError> {
    let mut result = Vec::new();

    let codec_uuids = encoder.get_encode_guids()?;
    for codec_uuid in codec_uuids.iter() {
        let codec =
            match match_codec(codec_uuid) {
                Some(c) => c,
                None => continue
            };

        let profile_uuids = encoder.get_encode_profile_guids(codec_uuid)?;
        let profiles = match_profiles(&profile_uuids);

        result.push(NvEncodeCapabilities {
            codec,
            profiles,
            max_width: encoder.get_encode_caps(codec_uuid, _NV_ENC_CAPS_NV_ENC_CAPS_WIDTH_MAX)?.try_into()?,
            max_height: encoder.get_encode_caps(codec_uuid, _NV_ENC_CAPS_NV_ENC_CAPS_HEIGHT_MAX)?.try_into()?,
            ten_bit_encode_supported: encoder.get_encode_caps(codec_uuid, _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_10BIT_ENCODE)? == 1,
            b_frames_supported: encoder.get_encode_caps(codec_uuid, _NV_ENC_CAPS_NV_ENC_CAPS_NUM_MAX_BFRAMES)? > 0,
            yuv_444_encode_supported: encoder.get_encode_caps(codec_uuid, _NV_ENC_CAPS_NV_ENC_CAPS_SUPPORT_YUV444_ENCODE)? == 1,
        })
    }

    Ok(result)
}

fn match_codec(uuid: &Uuid) -> Option<Codec> {
    return if uuid == &CODEC_H264 {
        Some(Codec::H264)
    } else if uuid == &CODEC_HEVC {
        Some(Codec::Hevc)
    } else if uuid == &CODEC_AV1 {
        Some(Codec::Av1)
    } else {
        eprintln!("Unknown codec GUID: {}", uuid);
        None
    };
}

fn match_profiles(uuids: &[Uuid]) -> Vec<EncodeProfile> {
    uuids.iter().filter_map(|uuid| {
        if uuid == &H264_PROFILE_BASELINE {
            Some(EncodeProfile::Baseline)
        } else if uuid == &H264_PROFILE_MAIN {
            Some(EncodeProfile::Main)
        } else if uuid == &H264_PROFILE_HIGH {
            Some(EncodeProfile::High)
        } else if uuid == &H264_PROFILE_HIGH_444 {
            Some(EncodeProfile::High444)
        } else if uuid == &HEVC_PROFILE_MAIN {
            Some(EncodeProfile::Main)
        } else if uuid == &HEVC_PROFILE_MAIN10 {
            Some(EncodeProfile::Main10)
        } else if uuid == &AV1_PROFILE_MAIN {
            Some(EncodeProfile::Main)
        } else {
            let (first_field, ..) = uuid.as_fields();
            if !IGNORED_PROFILES.contains(&first_field) {
                eprintln!("Unknown profile GUID: {}", uuid);
            }

            None
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::caps::encode::get_encode_capabilities;
    use crate::context::CudaContext;
    use crate::device::enumerate_devices;
    use crate::dylib::is_cuda_loaded;
    use crate::encoder::NvEncoder;

    #[test]
    fn test_get_encode_capabilities() -> Result<(), NvidiaError> {
        if !is_cuda_loaded() {
            eprintln!("libcuda.so not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        assert!(!devices.is_empty());

        let context = CudaContext::new(devices.get(0).unwrap())?;

        context.with_floating_ctx(|context| {
            let encoder = NvEncoder::new(context)?;

            let encode_capabilities = get_encode_capabilities(&encoder)?;

            dbg!(&encode_capabilities);
            assert!(!encode_capabilities.is_empty());

            Ok(())
        })?;

        Ok(())
    }
}
