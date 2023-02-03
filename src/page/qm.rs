use dioxus::prelude::*;

use crate::components::{Card, CardItem};

pub fn qm(cx: Scope) -> Element {
    render!(
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content.css"
        }
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content-card.css"
        }
        main{
            div {
                class: "content-wrapper",
                Card{
                    title:"卡片测试",
                    CardItem{
                        name: String::from("卡片1"),
                        description: String::from("一些文字")
                    }
                    CardItem{
                        name: String::from("卡片2"),
                        description: String::from("一些文字")
                    }
                    CardItem{
                        name: String::from("卡片3"),
                        description: String::from("一些文字")
                    }
                }
            }
        }
    )
}
