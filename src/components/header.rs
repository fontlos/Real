use dioxus::prelude::*;

use crate::APP;

// 颜色主题图标
const THEME: &str = "M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z";

#[cfg(not(target_arch = "wasm32"))]
pub fn header(cx: Scope) -> Element {
    let app_mode = use_read(&cx, APP);
    let app_mode_set = use_set(&cx, APP);
    let is_wide = use_state(&cx,||"");

    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/header.css"
        }
        header{
            class:"{is_wide}",
            div {
                class: "config",
                img {
                    class: "account-img",
                    src: "assets/img/logo.png",
                    alt: "",
                }
                div {
                    class: "dark-light",
                    onclick:move|_|{
                        if app_mode == "dark"{
                            app_mode_set(String::from("light"))
                        }else {
                            app_mode_set(String::from("dark"))
                        }
                    },
                    svg {
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        fill: "none",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke: "currentColor",
                        path {
                            d: "{THEME}",
                        }
                    }
                }
            }
            windows_drag()
            div { class: "header-menu",
                a {
                    class: "menu-button active  notify",
                    href: "#",
                    "首页"
                }
                a {
                    class: "menu-button",
                    href: "#",
                    "歌单"
                }
                a {
                    class: "menu-button",
                    href: "#",
                    "本地文件夹"
                }
                a {
                    class: "menu-button",
                    href: "#",
                    "设置"
                }
            }
            div {
                class: "search-bar",
                input {
                    r#type: "text",
                    placeholder: "Search",
                    onfocusin:move|_|{
                        is_wide.set("wide")
                    },
                    onfocusout:move|_|{
                        is_wide.set("")
                    },
                }
            }
            windows_drag()
            control_button()
        }
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn control_button(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);
    let is_max = use_state(&cx, || false);
    rsx!(cx,
        div {
            class: "control-buttons",
            div{
                class: "control-button minimize",
                onclick:move|_|{
                    window.set_minimized(true);
                }
            }
            div{
                class: "control-button maximize",
                onclick:move|_|{
                    if !is_max{
                        window.toggle_maximized();
                    }else{
                        window.set_maximized(false);
                    }
                }
            }
            div{
                class: "control-button close",
                onclick:move|_|{
                    window.close();
                }
            }
        }
    )
}

#[cfg(target_arch = "wasm32")]
fn control_button(cx: Scope) -> Element {
    rsx!(cx,
        div {
            class: "control-buttons",
            div{
                class: "control-button minimize",
                background_color: "#5fcf65",
            }
            div{
                class: "control-button maximize",
                background_color: "#f8ce52",
            }
            div{
                class: "control-button close",
                background_color: "#f96057",
            }
        }
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn windows_drag(cx: Scope) -> Element {
    let window = dioxus::desktop::use_window(&cx);
    rsx!(cx,
        div{
            class:"windows-drag",
            onmousedown:move|_|{
                window.drag();
            },
        }
    )
}

#[cfg(target_arch = "wasm32")]
fn windows_drag(cx: Scope) -> Element {
    rsx!(cx,
        div{
            class:"windows-drag",
        }
    )
}