use dioxus::prelude::*;

pub fn IndexMenu(cx: Scope) -> Element {
    render!(
        link{
            rel:"stylesheet",
            href:"assets/css/components/menu/index-menu.css"
        }
        div {
            class: "index-menu",
            a {
                class: "link",
                href: "#",
                "标题栏"
            }
            div {
                class: "menu",
                a {
                    class: "menu-link active",
                    href: "#",
                    "内部导航栏1"
                }
                a {
                    class: "menu-link",
                    href: "#",
                    "内部导航栏2"
                }
                a {
                    class: "menu-link",
                    href: "#",
                    "内部导航栏3"
                }
            }
        }
    )
}

pub fn ScrollMenu(cx: Scope) -> Element {
    render!(
        link{
            rel:"stylesheet",
            href:"assets/css/components/menu/scroll-menu.css"
        }
        div { class: "scroll-menu",
            div { class: "context",
                h3 { class: "img-content",
                    img {
                        src: "assets/img/logo.png",
                        alt: "",
                    }
                    "Solitude"
                }
                div { class: "description",
                    "实验性聚合型音乐播放器"
                }
                button { class: "content-sub-header-button content-button",
                    "点击前往"
                }
            }
            img { class: "logo",
                src: "assets/img/logo.png",
                alt: "",
            }
        }
    )
}

// TODO pop menu