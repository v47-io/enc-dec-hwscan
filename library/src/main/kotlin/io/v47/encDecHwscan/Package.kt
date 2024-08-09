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
package io.v47.encDecHwscan

import io.v47.encDecHwscan.exceptions.ConversionFailedException
import io.v47.encDecHwscan.exceptions.CriticalErrorException
import io.v47.encDecHwscan.exceptions.DriverFailureException
import io.v47.encDecHwscan.exceptions.OperationFailedException
import io.v47.encDecHwscan.exceptions.UnrecognizedErrorException
import io.v47.encDecHwscan.model.Chroma
import io.v47.encDecHwscan.model.Codec
import io.v47.encDecHwscan.model.CodecDetails
import io.v47.encDecHwscan.model.ColorDepth
import io.v47.encDecHwscan.model.DecodingSpec
import io.v47.encDecHwscan.model.Device
import io.v47.encDecHwscan.model.Driver
import io.v47.encDecHwscan.model.EncodeProfile
import io.v47.encDecHwscan.model.EncodingSpec
import java.lang.foreign.MemorySegment
import java.lang.foreign.ValueLayout
import kotlin.io.path.Path
import io.v47.encDecHwscan.bindings.CodecDetails as RsCodecDetails
import io.v47.encDecHwscan.bindings.DecodingSpec as RsDecodingSpec
import io.v47.encDecHwscan.bindings.Device as RsDevice
import io.v47.encDecHwscan.bindings.EncDecDevices as RsEncDecDevices
import io.v47.encDecHwscan.bindings.EncodingSpec as RsEncodingSpec

fun scanDevices(): List<Device> {
    val (result, errno) =
        Native.scanDevices { memorySegment ->
            mapDevices(
                RsEncDecDevices.devices(memorySegment),
                RsEncDecDevices.num_devices(memorySegment)
            )
        }

    return result ?: throw mapException(errno)
}

private fun mapDevices(devices: MemorySegment, numDevices: Int) =
    (0 until numDevices)
        .asSequence()
        .map { i -> RsDevice.asSlice(devices, i.toLong()) }
        .map { device ->
            Device(
                Driver.fromNative(RsDevice.driver(device)),
                RsDevice.ordinal(device),
                RsDevice.path(device).readStringOrNull()?.let { Path(it) },
                RsDevice.name(device).readStringOrNull(),
                mapCodecDetails(
                    RsDevice.codecs(device),
                    RsDevice.num_codecs(device)
                )
            )
        }
        .toList()

private fun mapCodecDetails(codecs: MemorySegment, numCodecs: Int) =
    (0 until numCodecs)
        .asSequence()
        .map { i -> RsCodecDetails.asSlice(codecs, i.toLong()) }
        .map { codecDetails ->
            CodecDetails(
                Codec.fromNative(RsCodecDetails.codec(codecDetails)),
                mapDecodingSpecs(
                    RsCodecDetails.decoding_specs(codecDetails),
                    RsCodecDetails.num_decoding_specs(codecDetails)
                ),
                mapEncodingSpecs(
                    RsCodecDetails.encoding_specs(codecDetails),
                    RsCodecDetails.num_encoding_specs(codecDetails)
                ),
            )
        }.associateBy { it.codec }

private fun mapDecodingSpecs(decodingSpecs: MemorySegment, numDecodingSpecs: Int) =
    (0 until numDecodingSpecs)
        .asSequence()
        .map { i -> RsDecodingSpec.asSlice(decodingSpecs, i.toLong()) }
        .map { decodingSpec ->
            DecodingSpec(
                Chroma.fromNative(RsDecodingSpec.chroma(decodingSpec)),
                ColorDepth.fromNative(RsDecodingSpec.color_depth(decodingSpec)),
                RsDecodingSpec.max_width(decodingSpec),
                RsDecodingSpec.max_height(decodingSpec),
            )
        }
        .toList()

private fun mapEncodingSpecs(encodingSpecs: MemorySegment, numEncodingSpecs: Int) =
    (0 until numEncodingSpecs)
        .asSequence()
        .map { i -> RsEncodingSpec.asSlice(encodingSpecs, i.toLong()) }
        .map { encodingSpec ->
            EncodingSpec(
                Chroma.fromNative(RsEncodingSpec.chroma(encodingSpec)),
                ColorDepth.fromNative(RsEncodingSpec.color_depth(encodingSpec)),
                EncodeProfile.fromNative(RsEncodingSpec.profile(encodingSpec)),
                RsEncodingSpec.max_width(encodingSpec),
                RsEncodingSpec.max_height(encodingSpec),
                when (RsEncodingSpec.b_frames_supported(encodingSpec)) {
                    0 -> false
                    1 -> true
                    else -> null
                }
            )
        }
        .toList()

@Suppress("MagicNumber")
private fun mapException(errno: Int) =
    when (errno) {
        -666 -> CriticalErrorException()
        1 -> DriverFailureException()
        2 -> OperationFailedException()
        3 -> ConversionFailedException()
        else -> UnrecognizedErrorException(errno)
    }

private fun MemorySegment.readStringOrNull(): String? {
    if (address() == 0L) {
        return null
    }

    val len = strlen()
    val buf = ByteArray(len)
    MemorySegment.copy(this, ValueLayout.JAVA_BYTE, 0L, buf, 0, len)

    return String(buf, Charsets.UTF_8)
}

private const val NUL = 0x0.toByte()

private fun MemorySegment.strlen(): Int {
    for (i in 0..Int.MAX_VALUE) {
        val byte = getAtIndex(ValueLayout.JAVA_BYTE, i.toLong())
        if (byte == NUL)
            return i
    }

    throw IllegalArgumentException("No null terminator found")
}
