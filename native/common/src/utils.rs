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
use std::{mem, ptr};

pub fn vec_to_ptr<T>(mut values: Vec<T>) -> (*mut T, u32) {
    if values.is_empty() {
        return (ptr::null_mut(), 0);
    }

    values.shrink_to_fit();

    let len = values.len();
    let ptr = values.as_mut_ptr();

    mem::forget(values);

    (ptr, len as u32)
}

pub fn drop_vec<T>(ptr: *mut T, len: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}
