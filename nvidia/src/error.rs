/*
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
use thiserror::Error;

use crate::sys::libcuviddec_sys::cudaError_enum;

#[derive(Error, Debug)]
pub enum NvidiaError {
    #[error("Nvidia driver not available: {0}")]
    NotLoaded(#[from] &'static libloading::Error),
    #[error("Operation failed: {0}")]
    OperationFailed(cudaError_enum),
    #[error("Nvidia driver not available")]
    NotAvailable,
    #[error("Symbol not found in library: {0}")]
    SymbolNotFound(&'static str),
}
