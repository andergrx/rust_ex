#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dx_gui::app::AppData;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/hello/:id")]
    Hello { id: i32 },
}

// #[derive(PartialEq, Clone, Default)]
// struct AppData {
//     output: String,
//     count: u32,
//     //onclick: EventHandler<MouseEvent>,
// }


// const DIV_STYLE: &str = r#"div {{ color: #26b72b; background: #222222;}} "#;

#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "./assets/main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let app_data = use_signal(|| AppData::default());

    rsx! {
        div {
            class: "row",

            LeftSide { data: app_data }
            RightSide { data: app_data }
        }
    }
}

#[component]
fn LeftSide(mut data: Signal<AppData>) -> Element {

    rsx! {
        div {
            class: "left_side",

            button {
                onclick: move |event| {
                    process_button(event, data);
                },
                "My Button"
            }
 
            Link {
                to: Route::Hello {
                    id: 42
                },
                "Greeting?"
            }
        }
    }
}

fn process_button(event: Event<MouseData>, mut data: Signal<AppData>) {
    info!("Clicked! Event: {event:?}");

    data.write().count += 1;
    let cnt = data.read().count;
    data.write().output.push_str(
        format!("Count: {}", cnt).as_str());
}

#[component]
fn RightSide(data: Signal<AppData>) -> Element {
    rsx! {
        div {
            class: "right_side",

            textarea {
                "{data.read().output}"
            }
        }
    }
}

#[component]
fn Hello(id: i32) -> Element {

    rsx! {
        div {
            class: "left_side",

            "Hello World {id}"

            Link {
                to: Route::Home {},
                "Return"
            }
        }
    }

}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}
