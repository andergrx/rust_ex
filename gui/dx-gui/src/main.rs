#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use reqwest::{get, Response};
use serde::Deserialize;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/hello")]
    Hello {},
    #[route("/dog")]
    Dog {},
}

#[derive(PartialEq, Clone, Default, Debug)]
struct AppData {
    output: String,
    count: u32,
}

// const DIV_STYLE: &str = r#"div {{ color: #26b72b; background: #222222;}} "#;

#[component]
pub fn App() -> Element {
    rsx! {
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
            Menu {}
        }
    }
}

#[component]
fn LeftSide(mut data: Signal<AppData>) -> Element {
    rsx! {
        div {
            class: "left_side",

            Button {
                name: "Button 1",
                onclick: move |event| {
                    process_button(event, data);
                },
            }

            Button {
                name: "Click on Me",
                onclick: move |event| {
                    build_list(event, data);
                },
            }

            Button {
                name: "Clear",
                onclick: move |event| {
                    clear_output(event, data);
                },
            }

            RouteLinkHello{}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct AppButtonProps {
    onclick: EventHandler<MouseEvent>,
    name: &'static str,
}

#[component]
fn Button(props: AppButtonProps) -> Element {
    rsx! {
        div {
            class: "button",
            a {
                class: "round_shadow_button",
                onclick: move |event| {
                    props.onclick.call(event);
                },

                "{props.name}"
          }
        }
    }
}

fn process_button(event: Event<MouseData>, mut data: Signal<AppData>) {
    info!("Clicked! Event: {event:?}");

    data.write().count += 1;
    let cnt = data.read().count;
    data.write()
        .output
        .push_str(format!("Count: {} ", cnt).as_str());
}

fn build_list(event: Event<MouseData>, mut data: Signal<AppData>) {
    info!("Clicked! Event: {event:?}");

    let list = (1..=10)
        .map(|x| format!("List elem {x}"))
        .collect::<Vec<_>>();

    data.write().output.push_str(format!("{:?}", list).as_str());
}

fn clear_output(event: Event<MouseData>, mut data: Signal<AppData>) {
    info!("Clicked! Event: {event:?}");

    data.write().output.clear();
}

#[component]
fn RightSide(data: Signal<AppData>) -> Element {
    rsx! {
        div {
            class: "right_side",

            span {
                class: "output",
                "{data.read().output}"
            }
        }
    }
}

#[component]
fn Menu() -> Element {
    let mut is_active = use_signal(|| false);

    rsx! {

        if *is_active.read_unchecked()  {
            nav {
                ul {
                    class: "menu",
                    onmouseout: move |_| {
                        *is_active.write() = false;
                    },

                    MenuItem { name: "Home", is_active: is_active}
                    MenuItem { name: "About", is_active: is_active}
                    MenuItem { name: "Contact", is_active: is_active}
                }
            }
        } else {
            a {
                class: "menu",
                onmouseover: move |_| {
                    *is_active.write() = true;
                },
                "Menu"
            }
        }
    }
}

#[component]
fn MenuItem(name: &'static str, is_active: Signal<bool>) -> Element {
    rsx! {
        li { 
            onmouseover: move |_| {
                *is_active.write() = true;
            }, 
            "{name}"
        }
    }
}

#[component]
fn RouteLinkHello() -> Element {
    rsx! {
        Link {
            class: "link",
            to: Route::Hello {
            },
            "Greeting?"
        }
    }
}

#[component]
fn Hello() -> Element {
    rsx! {
        div {
            class: "blink-me center",

            "Ello Mate!"
        }

        div {
            class: "center",

            Link {
                class: "link link-center",
                to: Route::Dog {},
                "Dog?"
            }

            Link {
                class: "link link-center",
                to: Route::Home {},
                "Return"
            }
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

fn get_doge(event: Event<MouseData>) {
    info!("Clicked! Event: {event:?}");

    Route::Dog {};
}

#[component]
fn Dog() -> Element {
    let future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => rsx! {
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{response.message}",
                }
            }

            Link {
                class: "link",
                to: Route::Hello {},
                "Return"
            }
        },
        Some(Err(_)) => rsx! {
            div { class: "status", "Loading dogs failed" }
        },
        None => rsx! {
            div { class: "status", "Loading dogs..." }
        },
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="/assets/main.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
