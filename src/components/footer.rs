use dioxus::prelude::*;

pub fn footer(cx: Scope) -> Element {
    rsx!(cx,
        link{
            rel:"stylesheet",
            href:"assets/css/components/footer.css"
        }
        footer{
            div{
                class:"player",
                span{
                    class:"player-button shadow",
                    span{
                        class:"play"
                    }
                    span{
                        class:"pause"
                    }
                }
                span{class:"player-cricle-1"}
                span{class:"player-cricle-2"}
            }
        }
    )
}
