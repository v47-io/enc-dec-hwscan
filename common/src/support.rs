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
use std::ffi::c_char;
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Codec {
    Mpeg1 = 1,
    Mpeg2 = 2,
    Mpeg4 = 4,
    Vc1 = 7,
    H264 = 264,
    Hevc = 265,
    Vp8 = 8,
    Vp9 = 9,
    Av1 = 10,
}

impl Display for Codec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Chroma {
    Yuv420 = 420,
    Yuv422 = 422,
    Yuv444 = 444,
}

impl Display for Chroma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ColorDepth {
    Bit8 = 8,
    Bit10 = 10,
    Bit12 = 12,
}

impl Display for ColorDepth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// cbindgen:field-names=[codec, chroma, colorDepth]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SupportTriple(Codec, Chroma, ColorDepth);

impl Display for SupportTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.0, self.1, self.2)
    }
}

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DevicesInfo {
    devices: *const DeviceInfo,
    num_devices: u32,
}

impl DevicesInfo {
    pub fn new(devices: &[DeviceInfo]) -> Self {
        Self {
            devices: devices.as_ptr(),
            num_devices: u32::try_from(devices.len()).unwrap(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Driver {
    Reserved = 0,
    Nvidia = 1,
    Vaapi = 2,
}

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceInfo {
    driver: Driver,
    ordinal: u32,
    name: *const c_char,
    name_length: u32,
    path: *const c_char,
    path_length: u32,
    decoders: *const SupportTriple,
    num_decoders: u32,
    encoders: *const SupportTriple,
    num_encoders: u32,
}

impl DeviceInfo {
    pub fn new(
        driver: Driver,
        ordinal: u32,
        name: &str,
        path: &str,
        decoders: &[SupportTriple],
        encoders: &[SupportTriple]
    ) -> DeviceInfo {
        DeviceInfo {
            driver,
            ordinal,
            name: name.as_ptr() as *const c_char,
            name_length: name.bytes().len().try_into().unwrap(),
            path: path.as_ptr() as *const c_char,
            path_length: path.bytes().len().try_into().unwrap(),
            decoders: decoders.as_ptr(),
            num_decoders: decoders.len().try_into().unwrap(),
            encoders: encoders.as_ptr(),
            num_encoders: encoders.len().try_into().unwrap()
        }
    }
}
