#![allow(non_snake_case)]
#![allow(dead_code)]

mod interface;
mod components;
mod page;
mod data;
#[cfg(not(target_arch = "wasm32"))]
pub mod desktop_launch;
#[cfg(target_arch = "wasm32")]
pub mod web_launch;

mod test;

use fermi::Atom;

static APP: Atom<String> = |_| String::from("dark");