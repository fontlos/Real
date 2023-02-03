use dioxus::prelude::*;
use fermi::{use_read, use_init_atom_root};
use router::{Router, Route};

use anyhow::Result;

use crate::components;
use crate::page;

#[cfg(target_arch = "wasm32")]
pub fn start() -> Result<()> {
    web::launch(app);
    Ok(())
}

use crate::APP;

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let app_mode = use_read(&cx, APP);

    render!(
        div{
            class: "app {app_mode}",
            Router{
                components::header{}
                // Link 不能再 Router之外使用，nav 组件必须放入 Router 内
                components::nav{}
                Route{
                    to: "/",
                    page::home{}
                }
                Route{
                    to: "/ncm",
                    page::ncm{}
                }
                Route{
                    to: "/ncm/:id",
                    page::ncm{}
                }
                Route{
                    to: "/qm",
                    page::qm{}
                }
                Route{
                    to: "/qm/:id",
                    page::qm{}
                }
                Route{
                    to: "/playlist/:id",
                    page::playlist{}
                }
                Route{
                    to: "/local/:id",
                    page::local{}
                }
                Route{
                    to: "",
                    page::home{}
                }
                components::footer{}
            }
        }
    )
}