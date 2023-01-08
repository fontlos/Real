extern crate embed_resource;
use std::env;

#[cfg(target_os = "windows")]
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("icons/icon.rc");
    }
}

#[cfg(all(not(target_os = "windows"), not(target_arch = "wasm32")))]
fn main() {}

#[cfg(all(not(target_os = "windows"), target_arch = "wasm32"))]
fn main() {}
