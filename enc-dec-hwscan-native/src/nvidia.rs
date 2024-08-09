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

use common::{Codec, CodecDetails, Device, Driver};
use nvidia::caps::{get_decode_capabilities, get_encode_capabilities, CudaDecodeSpec};
use nvidia::context::CudaContext;
use nvidia::device::enumerate_devices;
use nvidia::encoder::NvEncoder;
use nvidia::NvidiaError;

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

pub fn get_nvidia_devices() -> Result<Vec<Device>, NvidiaError> {
    let mut result = Vec::new();

    let devices = enumerate_devices()?;

    for device in devices {
        let ctx = CudaContext::new(&device)?;

        let mut decode_caps: HashMap<Codec, CodecDetails> = HashMap::new();
        let mut encode_caps: HashMap<Codec, CodecDetails> = HashMap::new();

        ctx.with_ctx(|| get_decode_capabilities(&CudaDecodeSpec::all()))?
            .into_iter()
            .for_each(|codec_details| {
                decode_caps.insert(codec_details.codec(), codec_details);
            });

        ctx.with_floating_ctx(|context| {
            let encoder = NvEncoder::new(context)?;

            get_encode_capabilities(&encoder)
        })?
        .into_iter()
        .for_each(|codec_details| {
            encode_caps.insert(codec_details.codec(), codec_details);
        });

        let codec_details = ALL_CODECS
            .iter()
            .filter_map(|codec| {
                let decode_caps = decode_caps.remove(codec);
                let encode_caps = encode_caps.remove(codec);

                if decode_caps.is_none() && encode_caps.is_none() {
                    None
                } else {
                    Some(CodecDetails::combine(*codec, decode_caps, encode_caps))
                }
            })
            .collect::<Vec<_>>();

        result.push(Device::new_with_ordinal(
            Driver::Nvidia,
            device.handle as u8,
            device.name,
            codec_details,
        ))
    }

    Ok(result)
}
