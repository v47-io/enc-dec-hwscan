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
use std::ptr::drop_in_place;

pub use common::*;

#[no_mangle]
pub extern "C" fn drop_devices_info(ptr: *mut DevicesInfo) {
    unsafe {
        drop_in_place(ptr);
    }
}

#[no_mangle]
pub extern "C" fn find_devices(result: *mut DevicesInfo) -> i32 {
    unsafe { *result = DevicesInfo::new(&[]) }

    0
}
