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
use std::ffi::{c_char, CString};
use std::ptr::slice_from_raw_parts_mut;

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

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Chroma {
    Monochrome = 0,
    Yuv420 = 420,
    Yuv422 = 422,
    Yuv444 = 444,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ColorDepth {
    Bit8 = 8,
    Bit10 = 10,
    Bit12 = 12,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EncodeProfile {
    Baseline = 1,
    Main = 10,
    Main10 = 11,
    High = 100,
    High444 = 101,
}

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodecSupport {
    codec: Codec,
    resolutions: *mut u32,
    num_resolutions: u32,
    chromas: *mut Chroma,
    num_chromas: u32,
    color_depths: *mut ColorDepth,
    num_color_depths: u32,
}

impl CodecSupport {
    pub fn new(
        codec: Codec,
        resolutions: Vec<u32>,
        chromas: Vec<Chroma>,
        color_depths: Vec<ColorDepth>,
    ) -> CodecSupport {
        let (resolutions, num_resolutions) = vec_to_ptr(resolutions);
        let (chromas, num_chromas) = vec_to_ptr(chromas);
        let (color_depths, num_color_depths) = vec_to_ptr(color_depths);

        CodecSupport {
            codec,
            resolutions,
            num_resolutions,
            chromas,
            num_chromas,
            color_depths,
            num_color_depths,
        }
    }
}

impl Drop for CodecSupport {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.resolutions, self.num_resolutions as usize));
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.chromas, self.num_chromas as usize));
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.color_depths, self.num_color_depths as usize));
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SupportInfo {
    devices: *mut Device,
    num_devices: u32,
}

impl SupportInfo {
    pub fn new(devices: Vec<Device>) -> Self {
        let (devices, num_devices) = vec_to_ptr(devices);

        Self {
            devices,
            num_devices,
        }
    }
}

impl Drop for SupportInfo {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.devices, self.num_devices as usize));
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
pub struct Device {
    driver: Driver,
    ordinal: u32,
    name: *mut c_char,
    path: *mut c_char,
    decoders: *mut CodecSupport,
    num_decoders: u32,
    encoders: *mut CodecSupport,
    num_encoders: u32,
}

impl Device {
    pub fn new(
        driver: Driver,
        ordinal: u32,
        name: String,
        path: String,
        decoders: Vec<CodecSupport>,
        encoders: Vec<CodecSupport>,
    ) -> Device {
        let (decoders, num_decoders) = vec_to_ptr(decoders);
        let (encoders, num_encoders) = vec_to_ptr(encoders);

        Device {
            driver,
            ordinal,
            name: CString::new(name).unwrap().into_raw(),
            path: CString::new(path).unwrap().into_raw(),
            decoders,
            num_decoders,
            encoders,
            num_encoders,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.name);
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.decoders, self.num_decoders as usize));
            let _ = Box::from_raw(slice_from_raw_parts_mut(self.encoders, self.num_encoders as usize));
        }
    }
}

fn vec_to_ptr<T>(values: Vec<T>) -> (*mut T, u32) {
    let len = values.len();
    let boxed = values.into_boxed_slice();
    let ptr = Box::into_raw(boxed) as *mut _;

    (ptr, len as u32)
}
