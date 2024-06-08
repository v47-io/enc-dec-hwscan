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
package io.v47.encDecHwscan.exceptions

sealed class EncDecHwscanException(message: String? = null) : Exception(message)

class CriticalErrorException : EncDecHwscanException()
class DriverFailureException : EncDecHwscanException()
class OperationFailedException : EncDecHwscanException()
class ConversionFailedException : EncDecHwscanException()
class UnrecognizedErrorException(errno: Int) : EncDecHwscanException("Unknown error: $errno")
