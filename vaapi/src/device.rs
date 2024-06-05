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
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::error::VaError;

const DEV_PATH: &'static str = "/dev/dri";
const DEV_BY_PATH_PATH: &'static str = "/dev/dri/by-path";

pub fn enumerate_devices() -> Result<Vec<PathBuf>, VaError> {
    let dev_path = Path::new(DEV_PATH);
    let dev_by_path_path = Path::new(DEV_BY_PATH_PATH);

    if dev_by_path_path.is_dir() {
        let mut result = Vec::new();

        match read_dir(dev_by_path_path) {
            Ok(dir_iter) => {
                for dir_entry in dir_iter {
                    match dir_entry {
                        Ok(dir_entry) => {
                            if dir_entry.file_name().to_string_lossy().ends_with("render") {
                                result.push(dir_entry.path());
                            }
                        }
                        Err(err) => return Err(VaError::FailedToEnumerateDevices(err))
                    }
                }
            }
            Err(err) => return Err(VaError::FailedToEnumerateDevices(err))
        }

        if !result.is_empty() {
            return Ok(result);
        }
    }

    if dev_path.is_dir() {
        let mut result = Vec::new();

        match read_dir(dev_path) {
            Ok(dir_iter) => {
                for dir_entry in dir_iter {
                    match dir_entry {
                        Ok(dir_entry) => {
                            if dir_entry.file_name().to_string_lossy().starts_with("render") {
                                result.push(dir_entry.path());
                            }
                        }
                        Err(err) => return Err(VaError::FailedToEnumerateDevices(err))
                    }
                }
            }
            Err(err) => return Err(VaError::FailedToEnumerateDevices(err))
        }

        return Ok(result);
    }

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_devices() -> Result<(), VaError> {
        let devices = enumerate_devices()?;

        dbg!(&devices);

        Ok(())
    }
}
