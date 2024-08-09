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
use lazy_static::lazy_static;
use libloading::{Error, Library};

use crate::VaError;

lazy_static! {
    static ref _LIBVA_RAW: Result<Library, Error> = unsafe {
        match Library::new("libva.so.2") {
            Ok(lib) => Ok(lib),
            Err(_) => Library::new("libva.so"),
        }
    };
    pub static ref LIBVA: Result<&'static Library, &'static Error> = _LIBVA_RAW.as_ref();
    static ref _LIBVA_DRM_RAW: Result<Library, Error> = unsafe {
        match Library::new("libva-drm.so.2") {
            Ok(lib) => Ok(lib),
            Err(_) => Library::new("libva-drm.so"),
        }
    };
    pub static ref LIBVA_DRM: Result<&'static Library, &'static Error> = _LIBVA_DRM_RAW.as_ref();
}

#[cfg(test)]
pub(crate) fn is_va_loaded() -> bool {
    LIBVA.is_ok() && LIBVA_DRM.is_ok()
}

#[derive(Copy, Clone)]
pub struct Libs {
    pub libva: &'static Library,
    pub libva_drm: &'static Library,
}

pub fn ensure_available() -> Result<Libs, VaError> {
    Ok(Libs {
        libva: (*LIBVA)?,
        libva_drm: (*LIBVA_DRM)?,
    })
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! get_sym {
    ($lib_var:expr, $sym_name:ident) => {{
        use crate::VaError;

        unsafe {
            match $lib_var.get::<$sym_name>(stringify!($sym_name).as_bytes()) {
                Ok(sym) => sym,
                Err(_) => return Err(VaError::SymbolNotFound(stringify!($sym_name))),
            }
        }
    }};
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! varesult_call_sym {
    ($call: expr) => {{
        use crate::VaError;

        use crate::sys::va::VA_STATUS_SUCCESS;

        let result = unsafe { $call };
        if result != VA_STATUS_SUCCESS as i32 {
            return Err(VaError::from_status(result.try_into().unwrap())?);
        }
    }};
    ($self: expr, $func: ident ($($arg: expr),*)) => {{
        use crate::VaError;

        use crate::sys::va::VA_STATUS_SUCCESS;

        let fun = &$self.symbols.$func;
        let result = unsafe { fun($self.va_display, $($arg),*) };

        if result != VA_STATUS_SUCCESS as i32 {
            return Err(VaError::from_status(result.try_into().unwrap())?)
        }
    }};
}
