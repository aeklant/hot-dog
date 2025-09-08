use dioxus::prelude::*;
use hot_dog::frontend::DogView;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[route("/")]
    DogView,
}
