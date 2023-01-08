use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<'a> {
    title: &'a str,
    pub children: Element<'a>,
}

pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content-list.css"
        }
        div{
            class: "content-section",
            div {
                class: "content-section-title",
                "{cx.props.title}"
            }
            ul {
                class:"list",
                &cx.props.children
            }
        }
    )
}

#[derive(PartialEq, Props)]
pub struct SearchListItemProps {
    // 既是键，也是id
    id: u32,
    name: String,
    album_name: String,
    img: String,
}

pub fn SearchListItem(cx: Scope<SearchListItemProps>) -> Element {
    rsx!(cx,
        li {
            class: "list-item",
            div {
                class: "list-item-info",
                img {
                    src: "{cx.props.img}",
                    alt: "",
                }
                div{
                    class:"name",
                    "{cx.props.name}"
                }
                div{"author"}
                div{
                    class:"album",
                    "{cx.props.album_name}"
                }
            }
            span {
                class: "list-item-status",
                span {
                    class: "status-light green",
                }
                "Available"
            }
            div{
                class:"menu",
                div {
                    class:"dropdown",
                        a {
                            class:"menu-item",
                            href: "{cx.props.id}",
                            "下载"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "加入收藏"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "加入歌单"
                        }
                }
            }
        }
    )
}

#[derive(PartialEq, Props)]
pub struct PlayListItemProps {
    // 既是键，也是id
    id: u32,
    name: String,
    album_name: String,
    img: String,
}

pub fn PlayListItem(cx: Scope<PlayListItemProps>) -> Element {
    rsx!(cx,
        li {
            class: "list-item",
            div {
                class: "list-item-info",
                img {
                    src: "{cx.props.img}",
                    alt: "",
                }
                div{
                    class:"name",
                    "{cx.props.name}"
                }
                div{"author"}
                div{
                    class:"album",
                    "{cx.props.album_name}"
                }
            }
            span {
                class: "list-item-status",
                span {
                    class: "status-light green",
                }
                "Available"
            }
            div{
                class:"menu",
                div {
                    class:"dropdown",
                        a {
                            class:"menu-item",
                            href: "{cx.props.id}",
                            "下载"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "加入收藏"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "加入歌单"
                        }
                }
            }
        }
    )
}

#[derive(PartialEq, Props)]
pub struct LocalListItemProps {
    // 既是键，也是id
    url: String,
    name: String,
    author: String,
    album_name: String,
    img: String,
}

pub fn LocalListItem(cx: Scope<LocalListItemProps>) -> Element {
    rsx!(cx,
        li {
            class: "list-item",
            div {
                class: "list-item-info",
                img {
                    src: "{cx.props.img}",
                    alt: "",
                }
                div{
                    class:"name",
                    "{cx.props.name}"
                }
                div{"author"}
                div{
                    class:"album",
                    "{cx.props.album_name}"
                }
            }
            div{
                class:"menu",
                div {
                    class:"dropdown",
                        a {
                            class:"menu-item",
                            href: "#",
                            "加入播放列表"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "打开文件位置"
                        }
                        a {
                            class:"menu-item",
                            href: "#",
                            "文件详细信息"
                        }
                }
            }
        }
    )
}
