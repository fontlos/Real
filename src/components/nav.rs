use dioxus::prelude::*;

// use crate::CONFIG;

pub fn nav(cx: Scope) -> Element {
    // let config = use_read(&cx, CONFIG);
    // let playlists = config.playlists.iter().map(|list| {
    //     let id = list.id();
    //     let name = list.name();
    //     rsx!(NavPlayListItem {
    //         key: "{id}",
    //         title: name,
    //         link: format!("/playlist/{id}")
    //     })
    // });
    // let locals = config.locals.iter().map(|list| {
    //     let id = list.id();
    //     let name = list.name();
    //     rsx!(NavLocalItem {
    //         key: "{id}",
    //         title: name,
    //         link: format!("/local/{id}")
    //     })
    // });

    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/nav.css"
        }
        nav{
            div {
                class: "nav-item-group",
                div {
                    class: "item-group-title",
                    "发现"
                }
                div {
                    class: "nav-item",
                    NavItem{
                        title:"首页",
                        link:"/"
                    }
                    NavItem{
                        title:"芸",
                        link:"/ncm"
                    }
                }
            }
            div {
                class: "nav-item-group",
                div {
                    class: "item-group-title",
                    "歌单"
                }
                div {
                    class: "nav-item",
                    // playlists
                }
            }
            div {
                class: "nav-item-group",
                div {
                    class: "item-group-title",
                    "本地文件夹"
                }
                div {
                    class: "nav-item",
                    // locals
                }
            }
        }
    )
}

#[derive(PartialEq, Props)]
struct NavItemProps<'a> {
    title: &'a str,
    link: &'a str,
}

#[allow(non_snake_case)]
fn NavItem<'a>(cx: Scope<'a, NavItemProps<'a>>) -> Element {
    rsx!(cx,
        Link {
            to: "{cx.props.link}",
            svg {
                view_box: "0 0 512 512",
                g {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M0 0h128v128H0zm0 0M192 0h128v128H192zm0 0M384 0h128v128H384zm0 0M0 192h128v128H0zm0 0",
                    }
                }
                path {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    d: "M192 192h128v128H192zm0 0",
                }
                path {
                    d: "M384 192h128v128H384zm0 0M0 384h128v128H0zm0 0M192 384h128v128H192zm0 0M384 384h128v128H384zm0 0",
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                }
            }
            "{cx.props.title}"
        }
    )
}

#[derive(PartialEq, Props)]
struct NavPlayListItemProps {
    title: String,
    link: String,
}

#[allow(non_snake_case)]
fn NavPlayListItem(cx: Scope<NavPlayListItemProps>) -> Element {
    rsx!(cx,
        Link {
            to: "{cx.props.link}",
            svg {
                view_box: "0 0 512 512",
                g {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M0 0h128v128H0zm0 0M192 0h128v128H192zm0 0M384 0h128v128H384zm0 0M0 192h128v128H0zm0 0",
                    }
                }
                path {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    d: "M192 192h128v128H192zm0 0",
                }
                path {
                    d: "M384 192h128v128H384zm0 0M0 384h128v128H0zm0 0M192 384h128v128H192zm0 0M384 384h128v128H384zm0 0",
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                }
            }
            "{cx.props.title}"
        }
    )
}

#[derive(PartialEq, Props)]
struct NavLocalItemProps {
    title: String,
    link: String,
}

#[allow(non_snake_case)]
fn NavLocalItem(cx: Scope<NavLocalItemProps>) -> Element {
    rsx!(cx,
        Link {
            to: "{cx.props.link}",
            svg {
                view_box: "0 0 512 512",
                g {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M0 0h128v128H0zm0 0M192 0h128v128H192zm0 0M384 0h128v128H384zm0 0M0 192h128v128H0zm0 0",
                    }
                }
                path {
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                    d: "M192 192h128v128H192zm0 0",
                }
                path {
                    d: "M384 192h128v128H384zm0 0M0 384h128v128H0zm0 0M192 384h128v128H192zm0 0M384 384h128v128H384zm0 0",
                    fill: "currentColor",
                    xmlns: "http://www.w3.org/2000/svg",
                }
            }
            "{cx.props.title}"
        }
    )
}
