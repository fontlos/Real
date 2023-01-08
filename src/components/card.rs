use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps<'a> {
    title: &'a str,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content-card.css"
        }
        div{
            class: "content-section",
            div {
                class: "content-section-title",
                "{cx.props.title}"
            }
            div {
                class: "cards",
                &cx.props.children
            }
        }
    )
}

#[derive(PartialEq, Props)]
pub struct CardItemProps {
    name: String,
    description: String,
}

#[allow(non_snake_case)]
pub fn CardItem(cx: Scope<CardItemProps>) -> Element {
    rsx!(cx,
        div { class: "card-item",
            span {
                img {
                    src: "assets/img/logo.png",
                    alt: "",
                }
                "{cx.props.name}"
            }
            div {
                class: "card-item-subtext",
                "{cx.props.description}"
            }
            div {
                class: "card-item-buttons",
                button {
                    class: "content-button status-button",
                    "Update"
                }
                div {
                    class: "menu",
                }
            }
        }
    )
}
