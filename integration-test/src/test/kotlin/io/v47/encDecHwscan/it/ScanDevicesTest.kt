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

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured
import io.v47.utils.Constants
import org.apache.http.HttpStatus
import org.junit.jupiter.api.Test

@QuarkusTest
class ScanDevicesTest {
    @Test
    fun `it should return scanned devices`() {
        val devices = RestAssured
            .get(Constants.Api.V1 + "/devices")
            .then()
            .assertThat()
            .statusCode(HttpStatus.SC_OK)
            .extract()
            .body()
            .`as`(ScannedDevices::class.java)
            .devices

        println(devices)
    }
}
