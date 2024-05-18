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
extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=include/cuviddec.h");
    println!("cargo:rerun-if-changed=include/nvEncodeAPI.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    gen_bindings(&PathBuf::from("include/cuviddec.h"), &out_path);
    gen_bindings(&PathBuf::from("include/nvEncodeAPI.h"), &out_path);
}

fn gen_bindings(path: &Path, out_path: &Path) {
    let bindings = bindgen::Builder::default()
        .header(path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for {:?}", path));

    let header_file_name = path
        .file_name().unwrap().to_str().unwrap()
        .strip_suffix(".h").unwrap();

    let target_file_name = format!("{}.rs", header_file_name);

    bindings
        .write_to_file(out_path.join(&target_file_name))
        .unwrap_or_else(|_| panic!("Unable to write bindings to {}", target_file_name));
}
