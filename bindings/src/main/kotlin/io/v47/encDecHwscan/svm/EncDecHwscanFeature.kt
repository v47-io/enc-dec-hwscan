/**
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
package io.v47.encDecHwscan.svm

import io.v47.encDecHwscan.Native
import io.v47.encDecHwscan.bindings.EncDecHwscan
import org.graalvm.nativeimage.hosted.Feature
import org.graalvm.nativeimage.hosted.Feature.DuringSetupAccess
import org.graalvm.nativeimage.hosted.RuntimeForeignAccess

@Suppress("unused")
internal class EncDecHwscanFeature : Feature {
    override fun duringSetup(access: DuringSetupAccess) {
        Native.load()

        RuntimeForeignAccess.registerForDowncall(EncDecHwscan.`scan_devices$descriptor`())
        RuntimeForeignAccess.registerForDowncall(EncDecHwscan.`free_devices$descriptor`())
    }
}
