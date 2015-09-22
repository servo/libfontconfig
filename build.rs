/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate pkg_config;

use std::process::Command;
use std::env;

fn main() {
    // if the system version of fontconfig is at least 2.11.1, use it
    if pkg_config::Config::new().atleast_version("2.11.1").find("fontconfig").is_ok() {
        return;
    }

    assert!(Command::new("make")
        .args(&["-R", "-f", "makefile.cargo"])
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=fontconfig");
}
