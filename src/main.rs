use dioxus::prelude::*;
use hot_dog::components::{ Buttons, DogView };

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        DogView {}
        Buttons {}
    }
}

