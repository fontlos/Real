#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(non_snake_case)]

use dioxus::prelude::*;

mod interface;
mod components;
mod page;
mod data;
#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(target_arch = "wasm32")]
mod web;

mod test;

static APP: Atom<String> = |_| String::from("dark");

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    desktop::start().unwrap();
    #[cfg(target_arch = "wasm32")]
    web::start().unwrap();
}