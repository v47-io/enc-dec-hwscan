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
package io.v47.encDecHwscan.quarkus

import io.quarkus.arc.deployment.IgnoreSplitPackageBuildItem
import io.quarkus.deployment.annotations.BuildStep
import io.quarkus.deployment.annotations.ExecutionTime.RUNTIME_INIT
import io.quarkus.deployment.annotations.Record
import io.quarkus.deployment.builditem.FeatureBuildItem
import io.quarkus.deployment.builditem.NativeImageFeatureBuildItem
import io.quarkus.deployment.builditem.nativeimage.NativeImageResourceDirectoryBuildItem
import io.v47.encDecHwscan.quarkus.recorder.NativeLoadRecorder

internal class EncDecHwscanProcessor {
    @BuildStep
    fun feature() = FeatureBuildItem("ms47-enc-dec-hwscan")

    @BuildStep
    fun nativeFeature() = NativeImageFeatureBuildItem("io.v47.encDecHwscan.svm.EncDecHwscanFeature")

    @BuildStep
    fun nativeImageResources() = NativeImageResourceDirectoryBuildItem("META-INF/natives")

    @BuildStep
    @Record(RUNTIME_INIT)
    fun recordNativeLoad(recorder: NativeLoadRecorder) {
        recorder.load()
    }

    @BuildStep
    fun ignoreSplitPackages() =
        IgnoreSplitPackageBuildItem(listOf("io.v47.encDecHwscan"))
}
