/**
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
package io.v47.encDecHwscan.it

import io.quarkus.runtime.annotations.RegisterForReflection
import io.v47.encDecHwscan.model.Device
import io.v47.encDecHwscan.scanDevices
import io.v47.utils.Constants
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path
import jakarta.ws.rs.Produces
import jakarta.ws.rs.core.MediaType

@Path(Constants.Api.V1 + "/devices")
@Produces(MediaType.APPLICATION_JSON)
class ScanDevicesEndpoint {
    @GET
    fun getDevices() =
        ScannedDevices(scanDevices())
}

@RegisterForReflection
data class ScannedDevices(val devices: List<Device>)
