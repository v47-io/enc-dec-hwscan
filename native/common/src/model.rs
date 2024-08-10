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
use crate::utils::{drop_vec, vec_to_ptr};
use indenter::indented;
use std::ffi::{c_char, CStr, CString};
use std::fmt::{Debug, Formatter, Write};
use std::ptr::slice_from_raw_parts_mut;
use std::{mem, ptr};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Driver {
    Vaapi = 0,
    Nvidia = 1,
}

#[repr(C)]
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    High10 = 110,
    High12 = 112,
    High444 = 140,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct CodecDetails {
    codec: Codec,
    decoding_specs: *mut DecodingSpec,
    num_decoding_specs: u32,
    encoding_specs: *mut EncodingSpec,
    num_encoding_specs: u32,
}

impl CodecDetails {
    pub fn new(codec: Codec, decoding: Vec<DecodingSpec>, encoding: Vec<EncodingSpec>) -> Self {
        let (decoding_specs, num_decoding_specs) = vec_to_ptr(decoding);
        let (encoding_specs, num_encoding_specs) = vec_to_ptr(encoding);

        Self {
            codec,
            decoding_specs,
            num_decoding_specs,
            encoding_specs,
            num_encoding_specs,
        }
    }

    pub fn combine(
        codec: Codec,
        decoding: Option<CodecDetails>,
        encoding: Option<CodecDetails>,
    ) -> Self {
        let (decoding_specs, num_decoding_specs) = if let Some(decoding) = decoding {
            unsafe { decoding.into_raw_decoding_specs() }
        } else {
            (ptr::null_mut(), 0)
        };

        let (encoding_specs, num_encoding_specs) = if let Some(encoding) = encoding {
            unsafe { encoding.into_raw_encoding_specs() }
        } else {
            (ptr::null_mut(), 0)
        };

        Self {
            codec,
            decoding_specs,
            num_decoding_specs,
            encoding_specs,
            num_encoding_specs,
        }
    }

    pub fn codec(&self) -> Codec {
        self.codec
    }

    pub unsafe fn into_raw_decoding_specs(self) -> (*mut DecodingSpec, u32) {
        let Self {
            decoding_specs,
            num_decoding_specs,
            ..
        } = self;
        mem::forget(self);

        (decoding_specs, num_decoding_specs)
    }

    pub unsafe fn into_raw_encoding_specs(self) -> (*mut EncodingSpec, u32) {
        let Self {
            encoding_specs,
            num_encoding_specs,
            ..
        } = self;
        mem::forget(self);

        (encoding_specs, num_encoding_specs)
    }
}

impl Drop for CodecDetails {
    fn drop(&mut self) {
        if self.num_decoding_specs > 0 {
            drop_vec(self.decoding_specs, self.num_decoding_specs);
        }

        if self.num_encoding_specs > 0 {
            drop_vec(self.encoding_specs, self.num_encoding_specs);
        }
    }
}

impl Debug for CodecDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CodecDetails {{")?;
        writeln!(f, "  codec: {:?},", self.codec)?;

        write!(f, "  decoding_specs: ")?;

        if self.num_decoding_specs > 0 {
            writeln!(f, "[")?;

            let slice = unsafe {
                slice_from_raw_parts_mut(self.decoding_specs, self.num_decoding_specs as usize).as_ref().unwrap()
            };

            for decoding_spec in slice.iter() {
                writeln!(indented(f).with_str("    "), "{:?}", decoding_spec)?;
            }

            writeln!(f, "  ],")?;
        } else {
            writeln!(f, "[],")?;
        }

        write!(f, "  encoding_specs: ")?;

        if self.num_encoding_specs > 0 {
            writeln!(f, "[")?;

            let slice = unsafe {
                slice_from_raw_parts_mut(self.encoding_specs, self.num_encoding_specs as usize).as_ref().unwrap()
            };

            for encoding_spec in slice.iter() {
                writeln!(indented(f).with_str("    "), "{:?},", encoding_spec)?;
            }

            writeln!(f, "  ]")?;
        } else {
            writeln!(f, "[]")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodingSpec {
    pub chroma: Chroma,
    pub color_depth: ColorDepth,
    pub max_width: u32,
    pub max_height: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodingSpec {
    pub chroma: Chroma,
    pub color_depth: ColorDepth,
    pub profile: EncodeProfile,
    pub max_width: u32,
    pub max_height: u32,
    pub b_frames_supported: ThreeValue,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ThreeValue {
    False = 0,
    True = 1,
    Unknown = 2,
}

impl From<bool> for ThreeValue {
    fn from(value: bool) -> Self {
        if value {
            ThreeValue::True
        } else {
            ThreeValue::False
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct Device {
    driver: Driver,
    ordinal: u8,
    path: *mut c_char,
    name: *mut c_char,
    codecs: *mut CodecDetails,
    num_codecs: u32,
}

impl Device {
    pub fn new_with_ordinal(
        driver: Driver,
        ordinal: u8,
        name: String,
        codecs: Vec<CodecDetails>,
    ) -> Self {
        let name = CString::new(name).unwrap();
        let (codecs, num_codecs) = vec_to_ptr(codecs);

        Self {
            driver,
            ordinal,
            path: ptr::null_mut(),
            name: name.into_raw(),
            codecs,
            num_codecs,
        }
    }

    pub fn new_with_path(
        driver: Driver,
        path: String,
        name: Option<String>,
        codecs: Vec<CodecDetails>,
    ) -> Self {
        let path = CString::new(path).unwrap();
        let name = name.map(|it| CString::new(it).unwrap());
        let (codecs, num_codecs) = vec_to_ptr(codecs);

        Self {
            driver,
            ordinal: 0,
            path: path.into_raw() as *mut c_char,
            name: name
                .map(|it| it.into_raw())
                .unwrap_or_else(|| ptr::null_mut()),
            codecs,
            num_codecs,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            if self.path != ptr::null_mut() {
                let _ = CString::from_raw(self.path as *mut c_char);
            }

            if self.name != ptr::null_mut() {
                let _ = CString::from_raw(self.name as *mut c_char);
            }
        }

        if self.num_codecs > 0 {
            drop_vec(self.codecs, self.num_codecs);
        }
    }
}

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Device {{")?;
        writeln!(f, "  driver: {:?},", self.driver)?;
        writeln!(f, "  ordinal: {},", self.ordinal)?;
        write!(f, "  path: ")?;

        if self.path != ptr::null_mut() {
            writeln!(f, "{:?},", unsafe { CStr::from_ptr(self.path as *const c_char) })?;
        } else {
            writeln!(f, "null,")?;
        }

        write!(f, "  name: ")?;

        if self.name != ptr::null_mut() {
            writeln!(f, "{:?},", unsafe { CStr::from_ptr(self.name as *const c_char) })?;
        } else {
            writeln!(f, "null,")?;
        }

        write!(f, "  codecs: ")?;

        if self.num_codecs > 0 {
            writeln!(f, "[")?;

            let slice = unsafe {
                slice_from_raw_parts_mut(self.codecs, self.num_codecs as usize).as_ref().unwrap()
            };

            for codec in slice.iter() {
                writeln!(indented(f).with_str("    "), "{:?},", codec)?;
            }

            writeln!(f, "  ]")?;
        } else {
            writeln!(f, "[]")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct EncDecDevices {
    devices: *mut Device,
    num_devices: u32,
}

impl EncDecDevices {
    pub fn new(devices: Vec<Device>) -> Self {
        let (devices, num_devices) = vec_to_ptr(devices);

        Self {
            devices,
            num_devices,
        }
    }
}

impl Drop for EncDecDevices {
    fn drop(&mut self) {
        if self.num_devices > 0 {
            drop_vec(self.devices, self.num_devices);
        }
    }
}

impl Debug for EncDecDevices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EncDecDevices {{")?;
        write!(f, "  devices: ")?;

        if self.num_devices > 0 {
            writeln!(f, "[")?;

            let slice = unsafe {
                slice_from_raw_parts_mut(self.devices, self.num_devices as usize).as_ref().unwrap()
            };

            for dev in slice.iter() {
                writeln!(indented(f).with_str("    "), "{:?},", dev)?;
            }

            writeln!(f, "  ]")?;
        } else {
            writeln!(f, "[]")?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}
