#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(dead_code)]
use fermi::Atom;

mod interface;
mod components;
mod page;
mod data;
#[cfg(not(target_arch = "wasm32"))]
mod desktop_launch;
#[cfg(target_arch = "wasm32")]
mod web_launch;

mod test;

static APP: Atom<String> = |_| String::from("dark");

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    desktop_launch::start().unwrap();
    #[cfg(target_arch = "wasm32")]
    web_launch::start().unwrap();
}

// use dioxus::prelude::*;

// fn main() {
//     dioxus_desktop::launch(app)
// }

// fn app(cx: Scope) -> Element {
//     render!(
//         div{
//         }
//     )
// }