// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cmake;

use std::path::PathBuf;
use std::env;

use cmake::Config;

fn main() {
    if let Some(llvm_config) = env::var_os("LLVM_CONFIG") {
        let dst = Config::new("../compiler-rt")
            .define("COMPILER_RT_BUILD_SANITIZERS", "ON")
            .define("COMPILER_RT_BUILD_BUILTINS", "OFF")
            .define("COMPILER_RT_BUILD_XRAY", "OFF")
            .define("LLVM_CONFIG_PATH", llvm_config)
            .build_target("asan")
            .build();

        println!("cargo:rustc-link-search=native={}",
                 dst.join("build/lib/linux").display());
        println!("cargo:rustc-link-lib=static=clang_rt.asan-x86_64");

        let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let mut stack = src_dir.join("../compiler-rt")
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap())
            .filter(|e| &*e.file_name() != ".git")
            .collect::<Vec<_>>();
        while let Some(entry) = stack.pop() {
            let path = entry.path();
            if entry.file_type().unwrap().is_dir() {
                stack.extend(path.read_dir().unwrap().map(|e| e.unwrap()));
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
