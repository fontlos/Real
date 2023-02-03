use dioxus::prelude::*;

pub fn cube(cx: Scope) -> Element {
    let count = use_state(&cx, ||10);
    let cube = (1..*count.get()).into_iter().map(|index|{
        rsx!(
            div{
                class:"cube",
                style:"--count: {count}; --index: {index};",
                div{
                    class:"cube-part",
                    div{class:"cube-core",
                        div{
                            class:"cube-sides",
                            div{class:"cube-side"}
                            div{class:"cube-side"}
                            div{class:"cube-side"}
                            div{class:"cube-side"}
                            div{class:"cube-side"}
                            div{class:"cube-side"}
                        }
                    }
                }
            }
        )
    });
    render!(
        link{
            rel:"stylesheet",
            href:"assets/css/components/cube.css"
        }
        div{
            class:"content-section",
            div{
                class:"cube-wrapper",
                cube
            }
        }
    )
}