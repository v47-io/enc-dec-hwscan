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
package io.v47.encDecHwscan.model

import com.fasterxml.jackson.annotation.JsonAlias
import io.quarkus.runtime.annotations.RegisterForReflection
import java.nio.file.Path

@RegisterForReflection
data class DecodingSpec(
    val chroma: Chroma,
    val colorDepth: ColorDepth,
    val maxWidth: Int,
    val maxHeight: Int
)

@RegisterForReflection
data class EncodingSpec(
    val chroma: Chroma,
    val colorDepth: ColorDepth,
    val profile: EncodeProfile,
    val maxWidth: Int,
    val maxHeight: Int,
    @JsonAlias("bframesSupported")
    val bFramesSupported: Boolean?
)

@RegisterForReflection
data class CodecDetails(
    val codec: Codec,
    val decodingSpecs: List<DecodingSpec>,
    val encodingSpecs: List<EncodingSpec>,
)

@RegisterForReflection
data class Device(
    val driver: Driver,
    val ordinal: Byte?,
    val path: Path?,
    val name: String?,
    val codecs: Map<Codec, CodecDetails>
)
