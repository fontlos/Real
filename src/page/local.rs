use dioxus::prelude::*;

// use crate::components::list::{List, LocalListItem};
// use crate::config::data::LocalList;
// use crate::CONFIG;

pub fn local(cx: Scope) -> Element {
    let route = use_route(&cx);
    let id = match route.segment("id") {
        Some(val) => val.to_string(),
        None => "1".to_string(),
    };

    // let lists = use_read(&cx, CONFIG).locals.clone();

    // let local_lists = use_future(&cx, (), |_| async move {
    //     let list = LocalList {
    //         id: String::from("1"),
    //         name: String::from("音乐 库"),
    //         list_url: String::from("./config/local/音乐 库.json"),
    //         dir_url: String::from("D:/Library/Music"),
    //     };
    // for list in lists.iter() {
    //     if list.id() == id{
    //         let content = list.read();
    //         let name = content.name;
    //         let combine_item = if let Some(items) = content.items{
    //             let item = items.iter().map(|i|{
    //                 let name = i.name();
    //                 let author = i.author();
    //                 let album = i.album();
    //                 let url = i.url();
    //                 rsx!(LocalListItem{
    //                     name:name,
    //                     author:author,
    //                     album_name:album,
    //                     url:url,
    //                     img:String::from("#")
    //                 })
    //             });
    //             rsx!(item)
    //         }else{
    //             rsx!("nothing")
    //         };
    //     }
    // }
    // });

    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content.css"
        }
        link{
            rel:"stylesheet",
            href:"assets/css/components/content/content-card.css"
        }
        main{
            "{id}"
            // Card{
            //     title:"卡片测试",
            //     CardItem{
            //         name: String::from("卡片1"),
            //         description: String::from("一些文字")
            //     }
            //     CardItem{
            //         name: String::from("卡片2"),
            //         description: String::from("一些文字")
            //     }
            //     CardItem{
            //         name: String::from("卡片3"),
            //         description: String::from("一些文字")
            //     }
            // }
        }
    )
}
