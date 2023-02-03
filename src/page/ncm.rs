use dioxus::prelude::*;

use crate::components::{
    Card, CardItem,
    List, SearchListItem,
    IndexMenu, cube
};
use crate::data::search::search_ncm;

pub fn ncm(cx: Scope) -> Element {
    let search = search_ncm::SearchConfig {
        key: String::from("独角"),
        offset: 0,
        limit: 10,
        search_type: 1,
    };

    let result = use_future(&cx, (), |_| async move { search.search().await.unwrap() });

    let lists = match result.value() {
        Some(value) => {
            let list = value.iter().map(|song| {
                let id = song.id();
                let name = song.name();
                let album_name = song.album_name();
                let img = song.img();
                rsx!(SearchListItem {
                    // 键无法传给子组件，单独设置
                    key: "{id}",
                    id: id,
                    name: name,
                    album_name: album_name,
                    img: img,
                })
            });
            rsx!(
                List{
                    title:"搜索结果",
                    list
            })
        }
        None => rsx!(
            // link{
            //     rel:"stylesheet",
            //     href:"assets/css/components/content/content-loading.css"
            // }
            // div{
            //     class: "content-section",
            //     div{
            //         class: "loading",
            //         span{i{}}
            //     }
            // }
            cube::cube(cx)
        ),
    };

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
            IndexMenu{}
            div {
                class: "content-wrapper",
                lists
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
