/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::process::Command;

fn build_fontconfig() {
    assert!(Command::new("make")
        .env("MAKEFLAGS", env::var("CARGO_MAKEFLAGS").unwrap_or_default())
        .args(&["-R", "-f", "makefile.cargo"])
        .status()
        .unwrap()
        .success());

    println!(
        "cargo:rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=static=fontconfig");
    println!(
        "cargo:incdir={}",
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
}

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("android") {
        match system_deps::Config::new().probe() {
            Ok(libs) => {
                let libs = libs.get_by_name("fontconfig").unwrap();

                println!("cargo:rustc-link-lib=static=fontconfig");
                println!(
                    "cargo:incdir={}",
                    libs.include_paths[0]
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                );
            }
            Err(error) => {
                if cfg!(feature = "force_system_lib") {
                    panic!("{}", error.to_string());
                } else {
                    eprintln!("{}", error.to_string());
                    build_fontconfig();
                }
            }
        }
    } else {
        build_fontconfig();
    }
}
