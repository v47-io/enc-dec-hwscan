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
@file:Suppress("MagicNumber")

package io.v47.encDecHwscan.model

import io.v47.encDecHwscan.bindings.EncDecHwscan

enum class Driver(private val nativeValue: Int) {
    Vaapi(EncDecHwscan.Vaapi()),
    Nvidia(EncDecHwscan.Nvidia());

    companion object {
        internal fun fromNative(value: Int): Driver = entries.first { it.nativeValue == value }
    }
}

enum class Codec(private val nativeValue: Int) {
    Mpeg1(EncDecHwscan.Mpeg1()),
    Mpeg2(EncDecHwscan.Mpeg2()),
    Mpeg4(EncDecHwscan.Mpeg4()),
    Vc1(EncDecHwscan.Vc1()),
    H264(EncDecHwscan.H264()),
    Hevc(EncDecHwscan.Hevc()),
    Vp8(EncDecHwscan.Vp8()),
    Vp9(EncDecHwscan.Vp9()),
    Av1(EncDecHwscan.Av1());

    companion object {
        internal fun fromNative(value: Int): Codec = entries.first { it.nativeValue == value }
    }
}

enum class Chroma(private val nativeValue: Int) {
    Monochrome(EncDecHwscan.Monochrome()),
    Yuv420(EncDecHwscan.Yuv420()),
    Yuv422(EncDecHwscan.Yuv422()),
    Yuv444(EncDecHwscan.Yuv444());

    companion object {
        internal fun fromNative(value: Int): Chroma = entries.first { it.nativeValue == value }
    }
}

enum class ColorDepth(private val nativeValue: Int) {
    Bit8(EncDecHwscan.Bit8()),
    Bit10(EncDecHwscan.Bit10()),
    Bit12(EncDecHwscan.Bit12());

    companion object {
        internal fun fromNative(value: Int): ColorDepth = entries.first { it.nativeValue == value }
    }
}

enum class EncodeProfile(private val nativeValue: Int) {
    Baseline(EncDecHwscan.Baseline()),
    Main(EncDecHwscan.Main()),
    Main10(EncDecHwscan.Main10()),
    High(EncDecHwscan.High()),
    High10(EncDecHwscan.High10()),
    High12(EncDecHwscan.High12()),
    High444(EncDecHwscan.High444());

    companion object {
        internal fun fromNative(value: Int): EncodeProfile = entries.first { it.nativeValue == value }
    }
}
