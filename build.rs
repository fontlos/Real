extern crate embed_resource;

#[cfg(target_os = "windows")]
fn main() {
    embed_resource::compile("icons/icon.rc", embed_resource::NONE);
}

#[cfg(target_os = "macos")]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {}

#[cfg(target_arch = "wasm32")]
fn main() {}
