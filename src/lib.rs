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
use std::panic::catch_unwind;
use std::ptr::drop_in_place;

pub use common::*;

#[no_mangle]
pub extern "C" fn drop_support_info(ptr: *mut SupportInfo) {
    let _ = catch_unwind(|| {
        unsafe {
            drop_in_place(ptr);
        }
    }).inspect_err(|err| {
        eprintln!("Critical error in enc_dec_hwscan::drop_support_info: {:?}", err);
    });
}

#[no_mangle]
pub extern "C" fn detect_supported_devices(result: *mut SupportInfo) -> i32 {
    catch_unwind(|| {
        unsafe {
            *result = SupportInfo::new(vec![]);
        }

        0
    }).unwrap_or_else(|err| {
        eprintln!("Critical error in enc_dec_hwscan::detect_support: {:?}", err);
        -666
    })
}
