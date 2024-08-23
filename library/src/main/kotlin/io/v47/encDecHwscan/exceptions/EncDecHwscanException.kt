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
package io.v47.encDecHwscan.exceptions

sealed class EncDecHwscanException(message: String? = null) : Exception(message)

/**
 * Indicates an unexpected error occurred in the native library.
 */
class CriticalErrorException : EncDecHwscanException()

/**
 * Indicates that the native library was unable to query a device driver for information.
 */
class DriverFailureException : EncDecHwscanException()

/**
 * Indicates a generic operation failure in the native library.
 */
class OperationFailedException : EncDecHwscanException()

/**
 * Indicates a failure to convert the native representation to Kotlin/JVM types.
 */
class ConversionFailedException : EncDecHwscanException()

/**
 * Indicates that some other unrecognized error occurred in the native library.
 */
class UnrecognizedErrorException(errno: Int) : EncDecHwscanException("Unknown error: $errno")
