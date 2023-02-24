#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    Real::desktop_launch::start().unwrap();
    #[cfg(target_arch = "wasm32")]
    Real::web_launch::start().unwrap();
}