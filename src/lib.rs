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

pub use common::*;

#[no_mangle]
pub unsafe extern "C" fn drop_devices(ptr: *mut Devices) {
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn detect_devices(result: *mut *mut Devices) -> i32 {
    catch_unwind(|| {
        let devices = Box::new(Devices::new(vec![]));

        // make sure this is done last and only if errno is going to be 0
        *result = Box::into_raw(devices);
        0
    }).unwrap_or_else(|err| {
        eprintln!("Critical error in enc_dec_hwscan::detect_devices: {:?}", err);
        -666
    })
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_detect_devices() {
        unsafe {
            let mut target = ptr::null_mut::<Devices>();

            assert_eq!(0, detect_devices(&mut target));

            dbg!(&*target);

            drop_devices(target);
        }
    }
}
