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
use std::collections::HashMap;

use common::{
    Chroma, Codec, CodecDetails, ColorDepth, DecodingSpec, EncodeProfile, EncodingSpec, ThreeValue,
};

use crate::display::DrmDisplay;
use crate::sys::va::{
    VAConfigAttribType_VAConfigAttribMaxPictureHeight,
    VAConfigAttribType_VAConfigAttribMaxPictureWidth, VAEntrypoint_VAEntrypointEncSlice,
    VAEntrypoint_VAEntrypointVLD, VAProfile, VAProfile_VAProfileAV1Profile0,
    VAProfile_VAProfileAV1Profile1, VAProfile_VAProfileH264Baseline, VAProfile_VAProfileH264High,
    VAProfile_VAProfileH264High10, VAProfile_VAProfileH264Main, VAProfile_VAProfileHEVCMain,
    VAProfile_VAProfileHEVCMain10, VAProfile_VAProfileHEVCMain12,
    VAProfile_VAProfileHEVCMain422_10, VAProfile_VAProfileHEVCMain422_12,
    VAProfile_VAProfileHEVCMain444, VAProfile_VAProfileHEVCMain444_10,
    VAProfile_VAProfileHEVCMain444_12, VAProfile_VAProfileMPEG2Main, VAProfile_VAProfileMPEG4Main,
    VAProfile_VAProfileVC1Main, VAProfile_VAProfileVP8Version0_3, VAProfile_VAProfileVP9Profile0,
    VAProfile_VAProfileVP9Profile1, VAProfile_VAProfileVP9Profile2, VAProfile_VAProfileVP9Profile3,
};
use crate::VaError;

const ALL_CODECS: [Codec; 9] = [
    Codec::Mpeg1,
    Codec::Mpeg2,
    Codec::Mpeg4,
    Codec::Vc1,
    Codec::Vp8,
    Codec::Vp9,
    Codec::H264,
    Codec::Hevc,
    Codec::Av1,
];

pub fn get_capabilities(display: &DrmDisplay) -> Result<Vec<CodecDetails>, VaError> {
    let mut decode_caps: HashMap<Codec, Vec<DecodingSpec>> = HashMap::new();
    let mut encode_caps: HashMap<Codec, Vec<EncodingSpec>> = HashMap::new();

    for profile in display.query_profiles()? {
        let profile_specs = map_profile(profile);
        if profile_specs.is_empty() {
            continue;
        }

        let entrypoints = display.query_entrypoints(profile)?;

        if entrypoints.contains(&VAEntrypoint_VAEntrypointVLD) {
            let attribs = display.get_config_attributes(profile, VAEntrypoint_VAEntrypointVLD)?;
            let max_width = attribs
                .iter()
                .find(|&it| it.type_ == VAConfigAttribType_VAConfigAttribMaxPictureWidth)
                .map(|it| it.value)
                .unwrap_or_default();

            let max_height = attribs
                .iter()
                .find(|&it| it.type_ == VAConfigAttribType_VAConfigAttribMaxPictureHeight)
                .map(|it| it.value)
                .unwrap_or_default();

            profile_specs.iter().for_each(|profile_specs| {
                decode_caps
                    .entry(profile_specs.0)
                    .or_default()
                    .push(DecodingSpec {
                        chroma: profile_specs.2,
                        color_depth: profile_specs.3,
                        max_width,
                        max_height,
                    })
            });
        }

        if entrypoints.contains(&VAEntrypoint_VAEntrypointEncSlice) {
            let attribs =
                display.get_config_attributes(profile, VAEntrypoint_VAEntrypointEncSlice)?;

            let max_width = attribs
                .iter()
                .find(|&it| it.type_ == VAConfigAttribType_VAConfigAttribMaxPictureWidth)
                .map(|it| it.value)
                .unwrap_or_default();

            let max_height = attribs
                .iter()
                .find(|&it| it.type_ == VAConfigAttribType_VAConfigAttribMaxPictureHeight)
                .map(|it| it.value)
                .unwrap_or_default();

            profile_specs.iter().for_each(|profile_specs| {
                encode_caps
                    .entry(profile_specs.0)
                    .or_default()
                    .push(EncodingSpec {
                        chroma: profile_specs.2,
                        color_depth: profile_specs.3,
                        profile: profile_specs.1,
                        max_width,
                        max_height,
                        b_frames_supported: ThreeValue::Unknown,
                    })
            });
        }
    }

    Ok(ALL_CODECS
        .iter()
        .filter_map(|codec| {
            let decode_caps = decode_caps.remove(codec);
            let encode_caps = encode_caps.remove(codec);

            if decode_caps.is_none() && encode_caps.is_none() {
                None
            } else {
                Some(CodecDetails::new(
                    *codec,
                    decode_caps.unwrap_or_else(|| vec![]),
                    encode_caps.unwrap_or_else(|| vec![]),
                ))
            }
        })
        .collect())
}

fn map_profile(profile: VAProfile) -> Vec<(Codec, EncodeProfile, Chroma, ColorDepth)> {
    if profile == VAProfile_VAProfileMPEG2Main {
        vec![(
            Codec::Mpeg2,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileMPEG4Main {
        vec![(
            Codec::Mpeg4,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileH264Baseline {
        vec![(
            Codec::H264,
            EncodeProfile::Baseline,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileH264Main {
        vec![(
            Codec::H264,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileH264High {
        vec![(
            Codec::H264,
            EncodeProfile::High,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileVC1Main {
        vec![(
            Codec::Vc1,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileVP8Version0_3 {
        vec![(
            Codec::Vp8,
            EncodeProfile::Baseline,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain10 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main10,
            Chroma::Yuv420,
            ColorDepth::Bit10,
        )]
    } else if profile == VAProfile_VAProfileVP9Profile0 {
        vec![(
            Codec::Vp9,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileVP9Profile1 {
        vec![
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit8,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv422,
                ColorDepth::Bit8,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv444,
                ColorDepth::Bit8,
            ),
        ]
    } else if profile == VAProfile_VAProfileVP9Profile2 {
        vec![
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit10,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit12,
            ),
        ]
    } else if profile == VAProfile_VAProfileVP9Profile3 {
        vec![
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit10,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit12,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv422,
                ColorDepth::Bit10,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv422,
                ColorDepth::Bit12,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv444,
                ColorDepth::Bit10,
            ),
            (
                Codec::Vp9,
                EncodeProfile::Main,
                Chroma::Yuv444,
                ColorDepth::Bit12,
            ),
        ]
    } else if profile == VAProfile_VAProfileHEVCMain12 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main,
            Chroma::Yuv420,
            ColorDepth::Bit12,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain422_10 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main10,
            Chroma::Yuv422,
            ColorDepth::Bit10,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain422_12 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main,
            Chroma::Yuv422,
            ColorDepth::Bit12,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain444 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main,
            Chroma::Yuv444,
            ColorDepth::Bit8,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain444_10 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main10,
            Chroma::Yuv444,
            ColorDepth::Bit10,
        )]
    } else if profile == VAProfile_VAProfileHEVCMain444_12 {
        vec![(
            Codec::Hevc,
            EncodeProfile::Main,
            Chroma::Yuv444,
            ColorDepth::Bit12,
        )]
    } else if profile == VAProfile_VAProfileAV1Profile0 {
        vec![
            (
                Codec::Av1,
                EncodeProfile::Main,
                Chroma::Yuv420,
                ColorDepth::Bit8,
            ),
            (
                Codec::Av1,
                EncodeProfile::Main10,
                Chroma::Yuv420,
                ColorDepth::Bit10,
            ),
        ]
    } else if profile == VAProfile_VAProfileAV1Profile1 {
        vec![
            (
                Codec::Av1,
                EncodeProfile::High,
                Chroma::Yuv420,
                ColorDepth::Bit8,
            ),
            (
                Codec::Av1,
                EncodeProfile::High10,
                Chroma::Yuv420,
                ColorDepth::Bit10,
            ),
            (
                Codec::Av1,
                EncodeProfile::High,
                Chroma::Yuv444,
                ColorDepth::Bit8,
            ),
            (
                Codec::Av1,
                EncodeProfile::High10,
                Chroma::Yuv444,
                ColorDepth::Bit10,
            ),
        ]
    } else if profile == VAProfile_VAProfileH264High10 {
        vec![(
            Codec::H264,
            EncodeProfile::High10,
            Chroma::Yuv420,
            ColorDepth::Bit10,
        )]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::device::enumerate_devices;
    use crate::dylib::is_va_loaded;

    use super::*;

    #[test]
    fn test_get_capabilities() -> Result<(), VaError> {
        if !is_va_loaded() {
            eprintln!("libva-drm.so is not available");
            return Ok(());
        }

        let devices = enumerate_devices()?;
        if devices.is_empty() {
            eprintln!("No DRM devices found");
            return Ok(());
        }

        let drm_display = DrmDisplay::new(devices.get(0).unwrap())?;

        println!("Found device: {}", &drm_display.vendor);
        dbg!(&drm_display);

        let capabilities = get_capabilities(&drm_display)?;

        dbg!(&capabilities);
        assert!(!capabilities.is_empty());

        Ok(())
    }
}
