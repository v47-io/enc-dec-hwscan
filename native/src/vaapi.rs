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
use common::{Device, Driver};
use vaapi::caps::get_capabilities;
use vaapi::device::enumerate_devices;
use vaapi::display::DrmDisplay;
use vaapi::VaError;

pub fn get_vaapi_devices(nvidia_done: bool) -> Result<Vec<Device>, VaError> {
    let mut result = Vec::new();

    let devices = enumerate_devices()?;

    for device in devices {
        let display = DrmDisplay::new(&device)?;
        if nvidia_done && display.vendor.to_lowercase().contains("nvdec") {
            continue;
        }

        let codec_details = get_capabilities(&display)?;

        result.push(Device::new_with_path(
            Driver::Vaapi,
            device.to_string_lossy().to_string(),
            Some(display.vendor.clone()),
            codec_details,
        ))
    }

    Ok(result)
}
