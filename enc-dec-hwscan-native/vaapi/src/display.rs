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
use std::{ptr, slice};
use std::alloc::{alloc_zeroed, Layout, realloc};
use std::ffi::{c_int, c_uint, CStr};
use std::fs::{File, OpenOptions};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use libloading::Symbol;

use dylib_types::*;

use crate::dylib::{ensure_available, Libs};
use crate::sys::va::{VA_ATTRIB_NOT_SUPPORTED, VAConfigAttrib, VADisplay, VAEntrypoint, VAProfile};
use crate::VaError;

#[allow(non_camel_case_types, dead_code)]
mod dylib_types {
    use std::ffi::{c_char, c_int};

    use crate::sys::va::{VAConfigAttrib, VADisplay, VAEntrypoint, VAProfile, VAStatus};

    pub type vaGetDisplayDRM = unsafe extern fn(fd: c_int) -> VADisplay;
    pub type vaInitialize = unsafe extern fn(dpy: VADisplay, major_version: *mut c_int, minor_version: *mut c_int) -> VAStatus;
    pub type vaQueryVendorString = unsafe extern fn(dpy: VADisplay) -> *const c_char;
    pub type vaMaxNumProfiles = unsafe extern fn(dpy: VADisplay) -> c_int;
    pub type vaMaxNumEntrypoints = unsafe extern fn(dpy: VADisplay) -> c_int;
    // pub type vaMaxNumConfigAttributes = unsafe extern fn(dpy: VADisplay) -> c_int;
    pub type vaQueryConfigProfiles = unsafe extern fn(dpy: VADisplay, profile_list: *mut VAProfile, num_profiles: *mut c_int) -> VAStatus;
    pub type vaQueryConfigEntrypoints = unsafe extern fn(dpy: VADisplay, profile: VAProfile, entrypoint_list: *mut VAEntrypoint, num_entrypoints: *mut c_int) -> VAStatus;
    pub type vaGetConfigAttributes = unsafe extern fn(dpy: VADisplay, profile: VAProfile, entrypoint: VAEntrypoint, attrib_list: *mut VAConfigAttrib, num_attribs: c_int) -> VAStatus;
    pub type vaTerminate = unsafe extern fn(dpy: VADisplay) -> VAStatus;
}

#[derive(Debug)]
struct VaSymbols {
    va_max_num_profiles: Symbol<'static, vaMaxNumProfiles>,
    va_max_num_entrypoints: Symbol<'static, vaMaxNumEntrypoints>,
    // va_max_num_config_attributes: Symbol<'static, vaMaxNumConfigAttributes>,
    va_query_config_profiles: Symbol<'static, vaQueryConfigProfiles>,
    va_query_config_entrypoints: Symbol<'static, vaQueryConfigEntrypoints>,
    va_get_config_attributes: Symbol<'static, vaGetConfigAttributes>,
    va_terminate: Symbol<'static, vaTerminate>,
}

#[derive(Debug)]
pub struct DrmDisplay {
    _drm_file: File,
    va_display: VADisplay,
    pub path: PathBuf,
    pub vendor: String,
    pub version_major: u32,
    pub version_minor: u32,
    symbols: VaSymbols,
}

impl Drop for DrmDisplay {
    fn drop(&mut self) {
        let va_terminate = &self.symbols.va_terminate;
        unsafe { va_terminate(self.va_display) };
    }
}

impl From<&DrmDisplay> for VADisplay {
    fn from(value: &DrmDisplay) -> Self {
        value.va_display
    }
}

impl From<&mut DrmDisplay> for VADisplay {
    fn from(value: &mut DrmDisplay) -> Self {
        value.va_display
    }
}

macro_rules! call_sym {
    ($self: expr, $func: ident ($($arg: expr),*)) => {{
        let fun = &$self.symbols.$func;
        unsafe { fun($self.va_display, $($arg),*) }
    }};
}

impl DrmDisplay {
    pub fn new(device_path: &Path) -> Result<Self, VaError> {
        let Libs { libva, libva_drm } = ensure_available()?;

        let drm_file = match OpenOptions::new().read(true).write(true).open(device_path) {
            Ok(drm_file) => drm_file,
            Err(err) => return Err(VaError::FailedToOpenDevice(device_path.to_path_buf(), err))
        };

        let sym_va_get_display_drm = get_sym!(libva_drm, vaGetDisplayDRM);
        let va_display = unsafe { sym_va_get_display_drm(drm_file.as_raw_fd()) };

        if va_display == ptr::null_mut() {
            Err(VaError::FailedToGetDisplay(device_path.to_path_buf()))
        } else {
            let sym_va_initialize = get_sym!(libva, vaInitialize);
            let mut version_major: c_int = 0;
            let mut version_minor: c_int = 0;

            varesult_call_sym!(sym_va_initialize(va_display, &mut version_major, &mut version_minor));

            let sym_va_query_vendor_string = get_sym!(libva, vaQueryVendorString);
            let vendor_string = unsafe { sym_va_query_vendor_string(va_display) };
            let vendor =
                if vendor_string == ptr::null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(vendor_string) }.to_string_lossy().to_string()
                };

            Ok(DrmDisplay {
                _drm_file: drm_file,
                va_display,
                path: device_path.to_path_buf(),
                vendor,
                version_major: version_major as u32,
                version_minor: version_minor as u32,
                symbols: VaSymbols {
                    va_max_num_profiles: get_sym!(libva, vaMaxNumProfiles),
                    va_max_num_entrypoints: get_sym!(libva, vaMaxNumEntrypoints),
                    // va_max_num_config_attributes: get_sym!(libva, vaMaxNumConfigAttributes),
                    va_query_config_profiles: get_sym!(libva, vaQueryConfigProfiles),
                    va_query_config_entrypoints: get_sym!(libva, vaQueryConfigEntrypoints),
                    va_get_config_attributes: get_sym!(libva, vaGetConfigAttributes),
                    va_terminate: get_sym!(libva, vaTerminate),
                },
            })
        }
    }

    pub fn query_profiles(&self) -> Result<Vec<VAProfile>, VaError> {
        let max_num_profiles = call_sym!(self, va_max_num_profiles());
        if max_num_profiles < 0 {
            return Err(VaError::OperationFailed("vaMaxNumProfiles returned a negative value".to_string(), -1));
        }

        let max_num_profiles = max_num_profiles as usize;

        let array_ptr = alloc_array::<VAProfile>(max_num_profiles);
        let mut num_profiles: c_int = 0;

        varesult_call_sym!(self, va_query_config_profiles(array_ptr, &mut num_profiles));

        let array_ptr = realloc_array(array_ptr, num_profiles as usize, max_num_profiles);
        Ok(make_vec(array_ptr, num_profiles as usize))
    }

    pub fn query_entrypoints(&self, profile: VAProfile) -> Result<Vec<VAEntrypoint>, VaError> {
        let max_num_entrypoints = call_sym!(self, va_max_num_entrypoints());
        if max_num_entrypoints < 0 {
            return Err(VaError::OperationFailed("vaMaxNumEntrypoints returned a negative value".to_string(), -1));
        }

        let max_num_entrypoints = max_num_entrypoints as usize;

        let array_ptr = alloc_array::<VAEntrypoint>(max_num_entrypoints);
        let mut num_entrypoints: c_int = 0;

        varesult_call_sym!(self, va_query_config_entrypoints(profile, array_ptr, &mut num_entrypoints));

        let array_ptr = realloc_array(array_ptr, num_entrypoints as usize, max_num_entrypoints);
        Ok(make_vec(array_ptr, num_entrypoints as usize))
    }

    pub fn get_config_attributes(&self, profile: VAProfile, entrypoint: VAEntrypoint) -> Result<Vec<VAConfigAttrib>, VaError> {
        /*let max_num_config_attributes = call_sym!(self, va_max_num_config_attributes());
        if max_num_config_attributes < 0 {
            return Err(VaError::OperationFailedT("vaMaxNumConfigAttributes returned a negative value".to_string(), -1));
        }

        let max_num_config_attributes = max_num_config_attributes as usize;*/

        // I'd love to use vaMaxNumConfigAttributes to get the actual value, but it just returns 1.
        // So we are now using a hard-coded value after looking at the header where the highest value
        // is 56. So we have 57 potential attributes. And vaGetConfigAttributes accepts an array
        // with many items just fine.
        let max_num_config_attributes = 57;

        let array_ptr = alloc_array::<VAConfigAttrib>(max_num_config_attributes);
        let array_slice = unsafe { slice::from_raw_parts_mut(array_ptr, max_num_config_attributes) };
        for (i, raw_item) in array_slice.iter_mut().enumerate() {
            (*raw_item).type_ = i as c_uint;
            (*raw_item).value = VA_ATTRIB_NOT_SUPPORTED;
        }

        varesult_call_sym!(self, va_get_config_attributes(profile, entrypoint, array_ptr, max_num_config_attributes as c_int));

        Ok(
            make_vec(array_ptr, max_num_config_attributes)
                .into_iter()
                .filter(|it| it.value != VA_ATTRIB_NOT_SUPPORTED)
                .collect()
        )
    }
}

fn alloc_array<T: Sized>(num_items: usize) -> *mut T {
    let layout = Layout::array::<T>(num_items).unwrap();

    unsafe {
        alloc_zeroed(layout) as *mut T
    }
}

fn realloc_array<T: Sized>(array_ptr: *mut T, num_items: usize, max_size: usize) -> *mut T {
    let max_layout = Layout::array::<T>(max_size).unwrap();
    let new_layout = Layout::array::<T>(num_items).unwrap();

    unsafe {
        let result = realloc(array_ptr as *mut u8, max_layout, new_layout.size()) as *mut T;
        if result == ptr::null_mut() {
            array_ptr
        } else {
            result
        }
    }
}

fn make_vec<T: Sized>(array_ptr: *mut T, num_items: usize) -> Vec<T> {
    unsafe {
        Vec::from_raw_parts(array_ptr, num_items, num_items)
    }
}

#[cfg(test)]
mod tests {
    use crate::device::enumerate_devices;
    use crate::dylib::is_va_loaded;

    use super::*;

    #[test]
    fn test_create_drm_display() -> Result<(), VaError> {
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

        Ok(())
    }

    #[test]
    fn test_query_everything() -> Result<(), VaError> {
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

        dbg!(&drm_display);

        let profiles = drm_display.query_profiles()?;

        dbg!(&profiles);
        assert!(!profiles.is_empty());

        let entrypoints = drm_display.query_entrypoints(*profiles.first().unwrap())?;

        dbg!(&entrypoints);
        assert!(!entrypoints.is_empty());

        let config_attribs =
            drm_display
                .get_config_attributes(
                    *profiles.first().unwrap(),
                    *entrypoints.first().unwrap(),
                )?;

        dbg!(&config_attribs);
        assert!(!config_attribs.is_empty());

        Ok(())
    }
}
