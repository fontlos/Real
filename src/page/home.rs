use dioxus::prelude::*;

use crate::components::{Card, CardItem, List, IndexMenu, ScrollMenu};

pub fn home(cx: Scope) -> Element {
    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content.css"
        }
        main{
            IndexMenu{}
            div {
                class: "content-wrapper",
                ScrollMenu{}
                List{
                    title:"列表测试"
                }
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
