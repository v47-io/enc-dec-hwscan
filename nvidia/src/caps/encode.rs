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
use crate::encoder::NvEncoder;
use crate::NvidiaError;

#[derive(Debug)]
pub struct NvEncodeCapabilies {
    pub codec: NvEncodeCodec,
    pub profiles: Vec<NvEncodeProfile>,
    pub max_width: usize,
    pub max_height: usize,
    pub ten_bit_encode_supported: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NvEncodeCodec {
    H264,
    HEVC,
    AV1,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NvEncodeProfile {
    Baseline,
    Main,
    High,
}

pub fn get_encode_capabilities(encoder: &NvEncoder) -> Result<Vec<NvEncodeCapabilies>, NvidiaError> {
    todo!()
}

mod guid {
    use uuid::Uuid;

    use crate::sys::libnv_encode_api_sys::GUID;

    pub const CODEC_H264: Uuid = Uuid::from_bytes([0x6b, 0xc8, 0x27, 0x62, 0x4e, 0x63, 0x4c, 0xa4, 0xaa, 0x85, 0x1e, 0x50, 0xf3, 0x21, 0xf6, 0xbf]);
    pub const CODEC_HEVC: Uuid = Uuid::from_bytes([0x79, 0x0c, 0xdc, 0x88, 0x45, 0x22, 0x4d, 0x7b, 0x94, 0x25, 0xbd, 0xa9, 0x97, 0x5f, 0x76, 0x03]);
    pub const CODEC_AV1: Uuid = Uuid::from_bytes([0x0a, 0x35, 0x22, 0x89, 0x0a, 0xa7, 0x47, 0x59, 0x86, 0x2d, 0x5d, 0x15, 0xcd, 0x16, 0xd2, 0x54]);

    pub const H264_PROFILE_BASELINE: Uuid = Uuid::from_bytes([0x07, 0x27, 0xbc, 0xaa, 0x78, 0xc4, 0x4c, 0x83, 0x8c, 0x2f, 0xef, 0x3d, 0xff, 0x26, 0x7c, 0x6a]);
    pub const H264_PROFILE_MAIN: Uuid = Uuid::from_bytes([0x60, 0xb5, 0xc1, 0xd4, 0x67, 0xfe, 0x47, 0x90, 0x94, 0xd5, 0xc4, 0x72, 0x6d, 0x7b, 0x6e, 0x6d]);
    pub const H264_PROFILE_HIGH: Uuid = Uuid::from_bytes([0xe7, 0xcb, 0xc3, 0x09, 0x4f, 0x7a, 0x4b, 0x89, 0xaf, 0x2a, 0xd5, 0x37, 0xc9, 0x2b, 0xe3, 0x10]);

    pub const HEVC_PROFILE_MAIN: Uuid = Uuid::from_bytes([0xb5, 0x14, 0xc3, 0x9a, 0xb5, 0x5b, 0x40, 0xfa, 0x87, 0x8f, 0xf1, 0x25, 0x3b, 0x4d, 0xfd, 0xec]);
    pub const HEVC_PROFILE_MAIN10: Uuid = Uuid::from_bytes([0xfa, 0x4d, 0x2b, 0x6c, 0x3a, 0x5b, 0x41, 0x1a, 0x80, 0x18, 0x0a, 0x3f, 0x5e, 0x3c, 0x9b, 0xe5]);

    pub const AV1_PROFILE_MAIN: Uuid = Uuid::from_bytes([0x5f, 0x2a, 0x39, 0xf5, 0xf1, 0x4e, 0x4f, 0x95, 0x9a, 0x9e, 0xb7, 0x6d, 0x56, 0x8f, 0xcf, 0x97]);

    impl TryInto<Uuid> for GUID {
        type Error = uuid::Error;

        fn try_into(self) -> Result<Uuid, Self::Error> {
            let data1_bytes: [u8; 4] = self.Data1.to_be_bytes();
            let data2_bytes: [u8; 2] = self.Data2.to_be_bytes();
            let data3_bytes: [u8; 2] = self.Data3.to_be_bytes();

            let full_bytes_vec: Vec<&[u8]> = vec![&data1_bytes, &data2_bytes, &data3_bytes, &self.Data4];
            let full_bytes: Vec<u8> = full_bytes_vec.concat();

            Uuid::from_slice(&full_bytes)
        }
    }
}
